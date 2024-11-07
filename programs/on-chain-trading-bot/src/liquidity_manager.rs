use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::dex::{raydium::*, jupiter::*, serum::*};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidityRatio {
    pub dex: DexType,
    pub pool_id: Pubkey,
    pub target_ratio: u8, // Percentage (0-100)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidityHealth {
    pub total_value_locked: u64,
    pub utilization_rate: u8,
    pub imbalance_ratio: u8,
    pub risk_score: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum DexType {
    Raydium,
    Jupiter,
    Serum,
}

pub struct CrossDexLiquidityManager;

impl CrossDexLiquidityManager {
    // Rebalance liquidity across DEXs
    pub fn rebalance_liquidity(
        ctx: Context<RebalanceLiquidity>,
        target_ratios: Vec<LiquidityRatio>,
    ) -> Result<()> {
        // Verify total ratio equals 100%
        let total_ratio: u8 = target_ratios.iter().map(|r| r.target_ratio).sum();
        require!(total_ratio == 100, TradingBotError::InvalidRatios);

        // Get current liquidity distribution
        let current_distribution = Self::get_current_distribution(ctx.accounts)?;

        // Calculate required moves
        let moves = Self::calculate_rebalance_moves(
            current_distribution,
            target_ratios.clone(),
        )?;

        // Execute rebalancing moves
        for move_action in moves {
            Self::execute_liquidity_move(
                ctx.accounts.into(),
                move_action,
            )?;
        }

        // Update liquidity tracking
        Self::update_liquidity_tracking(
            ctx.accounts.liquidity_tracker,
            target_ratios,
        )?;

        Ok(())
    }

    // Monitor and adjust positions
    pub fn monitor_liquidity_health(
        ctx: Context<MonitorLiquidity>,
    ) -> Result<LiquidityHealth> {
        // Get current liquidity metrics
        let total_value = Self::calculate_total_value(ctx.accounts)?;
        let utilization = Self::calculate_utilization(ctx.accounts)?;
        let imbalance = Self::calculate_imbalance(ctx.accounts)?;

        // Calculate risk score
        let risk_score = Self::calculate_risk_score(
            utilization,
            imbalance,
            ctx.accounts.market_volatility,
        )?;

        // Check if rebalancing is needed
        if risk_score > ctx.accounts.risk_threshold {
            Self::trigger_rebalancing(ctx.accounts.clone())?;
        }

        Ok(LiquidityHealth {
            total_value_locked: total_value,
            utilization_rate: utilization,
            imbalance_ratio: imbalance,
            risk_score,
        })
    }

    // Optimize liquidity provision
    pub fn optimize_liquidity_provision(
        ctx: Context<OptimizeLiquidity>,
        pool_configs: Vec<PoolConfig>,
    ) -> Result<()> {
        for config in pool_configs {
            // Calculate optimal liquidity amount
            let optimal_amount = Self::calculate_optimal_liquidity(
                config.clone(),
                ctx.accounts.market_data.clone(),
            )?;

            // Adjust liquidity if needed
            if Self::needs_adjustment(config.clone(), optimal_amount)? {
                Self::adjust_pool_liquidity(
                    ctx.accounts.into(),
                    config,
                    optimal_amount,
                )?;
            }
        }

        Ok(())
    }

    // Helper functions
    fn get_current_distribution(
        accounts: &RebalanceLiquidity,
    ) -> Result<Vec<(DexType, u64)>> {
        let mut distribution = Vec::new();

        // Get Raydium liquidity
        let raydium_tvl = accounts.raydium_pools
            .iter()
            .map(|p| p.total_value_locked())
            .sum::<u64>();
        distribution.push((DexType::Raydium, raydium_tvl));

        // Get Jupiter liquidity
        let jupiter_tvl = accounts.jupiter_pools
            .iter()
            .map(|p| p.total_value_locked())
            .sum::<u64>();
        distribution.push((DexType::Jupiter, jupiter_tvl));

        // Get Serum liquidity
        let serum_tvl = accounts.serum_markets
            .iter()
            .map(|m| m.total_value_locked())
            .sum::<u64>();
        distribution.push((DexType::Serum, serum_tvl));

        Ok(distribution)
    }

    fn calculate_rebalance_moves(
        current: Vec<(DexType, u64)>,
        target: Vec<LiquidityRatio>,
    ) -> Result<Vec<LiquidityMove>> {
        let total_value: u64 = current.iter().map(|(_, v)| v).sum();
        let mut moves = Vec::new();

        for ratio in target {
            let target_amount = (total_value as u128 * ratio.target_ratio as u128 / 100) as u64;
            let current_amount = current
                .iter()
                .find(|(dex, _)| *dex == ratio.dex)
                .map(|(_, v)| *v)
                .unwrap_or(0);

            if current_amount < target_amount {
                moves.push(LiquidityMove {
                    dex: ratio.dex,
                    pool_id: ratio.pool_id,
                    amount: target_amount - current_amount,
                    direction: MoveDirection::Add,
                });
            } else if current_amount > target_amount {
                moves.push(LiquidityMove {
                    dex: ratio.dex,
                    pool_id: ratio.pool_id,
                    amount: current_amount - target_amount,
                    direction: MoveDirection::Remove,
                });
            }
        }

        Ok(moves)
    }

    fn execute_liquidity_move(
        ctx: Context<ExecuteMove>,
        move_action: LiquidityMove,
    ) -> Result<()> {
        match move_action.direction {
            MoveDirection::Add => {
                match move_action.dex {
                    DexType::Raydium => {
                        RaydiumDex::add_liquidity(
                            ctx.accounts.into(),
                            move_action.amount,
                            move_action.pool_id,
                        )?;
                    },
                    DexType::Jupiter => {
                        JupiterDex::add_liquidity(
                            ctx.accounts.into(),
                            move_action.amount,
                            move_action.pool_id,
                        )?;
                    },
                    DexType::Serum => {
                        SerumDex::add_liquidity(
                            ctx.accounts.into(),
                            move_action.amount,
                            move_action.pool_id,
                        )?;
                    },
                }
            },
            MoveDirection::Remove => {
                // Similar implementation for removing liquidity
            },
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RebalanceLiquidity<'info> {
    #[account(mut)]
    pub liquidity_tracker: Account<'info, LiquidityTracker>,
    #[account(mut)]
    pub raydium_pools: Vec<Account<'info, RaydiumPool>>,
    #[account(mut)]
    pub jupiter_pools: Vec<Account<'info, JupiterPool>>,
    #[account(mut)]
    pub serum_markets: Vec<Account<'info, SerumMarket>>,
    pub token_program: Program<'info, Token>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct MonitorLiquidity<'info> {
    #[account(mut)]
    pub liquidity_tracker: Account<'info, LiquidityTracker>,
    pub market_volatility: Account<'info, MarketVolatility>,
    pub risk_threshold: u8,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct OptimizeLiquidity<'info> {
    #[account(mut)]
    pub liquidity_tracker: Account<'info, LiquidityTracker>,
    pub market_data: Account<'info, MarketData>,
    pub owner: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidityMove {
    pub dex: DexType,
    pub pool_id: Pubkey,
    pub amount: u64,
    pub direction: MoveDirection,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum MoveDirection {
    Add,
    Remove,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PoolConfig {
    pub dex: DexType,
    pub pool_id: Pubkey,
    pub min_liquidity: u64,
    pub max_liquidity: u64,
    pub target_utilization: u8,
} 