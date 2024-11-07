pub mod state;
pub mod dex;
pub mod oracles;
pub mod bot_strategy;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{TradingState, StrategyState, PerformanceMetrics, RiskParameters};
use crate::dex::{DexType, DexRoute};
use crate::oracles::PythOracle;

declare_id!("3seUuDx9nQXF18sEtcyZBkrf4YQjxHJuYFS26JVn1ERK");

#[program]
pub mod trading_bot {
    use super::*;

    // Initialize a new trading bot strategy
    pub fn initialize_strategy(
        ctx: Context<InitializeStrategy>,
        strategy_id: String,
        strategy_type: StrategyType,
    ) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;
        strategy.owner = ctx.accounts.owner.key();
        strategy.strategy_id = strategy_id;
        strategy.strategy_type = strategy_type;
        strategy.is_active = false;
        strategy.total_trades = 0;
        strategy.created_at = Clock::get()?.unix_timestamp;
        strategy.execution_metrics = ExecutionMetrics::default();
        strategy.risk_metrics = RiskMetrics::default();
        Ok(())
    }

    // Execute strategy based on configuration
    pub fn execute_strategy(
        ctx: Context<ExecuteStrategy>,
        blocks: Vec<StrategyBlock>,
    ) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;
        require!(strategy.is_active, TradingBotError::StrategyInactive);

        // Track execution state
        let mut execution_state = ExecutionState::new();

        // Execute blocks in sequence
        for block in blocks {
            match block.block_type {
                BlockType::Trigger => {
                    Self::execute_trigger(ctx.accounts.clone(), &block, &mut execution_state)?;
                },
                BlockType::Action => {
                    Self::execute_action(ctx.accounts.clone(), &block, &mut execution_state)?;
                },
                BlockType::Condition => {
                    Self::execute_condition(ctx.accounts.clone(), &block, &mut execution_state)?;
                },
                BlockType::Loop => {
                    Self::execute_loop(ctx.accounts.clone(), &block, &mut execution_state)?;
                },
                BlockType::Exit => {
                    if Self::should_exit(&block, &execution_state)? {
                        break;
                    }
                }
            }
        }

        // Update strategy metrics
        strategy.update_metrics(&execution_state)?;

        Ok(())
    }

    // Update strategy configuration
    pub fn update_strategy(
        ctx: Context<UpdateStrategy>,
        new_config: StrategyConfig,
    ) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;
        require!(
            strategy.owner == ctx.accounts.owner.key(),
            TradingBotError::Unauthorized
        );

        strategy.config = new_config;
        Ok(())
    }

    // Toggle strategy active state
    pub fn toggle_strategy(ctx: Context<ToggleStrategy>) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;
        require!(
            strategy.owner == ctx.accounts.owner.key(),
            TradingBotError::Unauthorized
        );

        strategy.is_active = !strategy.is_active;
        Ok(())
    }
}

// Account validation structures
#[derive(Accounts)]
pub struct InitializeStrategy<'info> {
    #[account(init, payer = owner, space = StrategyState::LEN)]
    pub strategy: Account<'info, StrategyState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteStrategy<'info> {
    #[account(mut)]
    pub strategy: Account<'info, StrategyState>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: Verified in program
    pub price_feed: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateStrategy<'info> {
    #[account(mut)]
    pub strategy: Account<'info, StrategyState>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ToggleStrategy<'info> {
    #[account(mut)]
    pub strategy: Account<'info, StrategyState>,
    pub owner: Signer<'info>,
}

// Error definitions
#[error_code]
pub enum TradingBotError {
    #[msg("Strategy is not active")]
    StrategyInactive,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid trade conditions")]
    InvalidTradeConditions,
    #[msg("Price feed is stale")]
    StalePriceFeed,
    #[msg("Price unavailable")]
    PriceUnavailable,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid configuration")]
    InvalidConfiguration,
} 