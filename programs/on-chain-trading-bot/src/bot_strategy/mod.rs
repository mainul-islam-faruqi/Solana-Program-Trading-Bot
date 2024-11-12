use anchor_lang::prelude::*;
use crate::dex::{raydium::*, jupiter::*, serum::*};
use crate::oracles::PythOracle;
use crate::state::{Strategy, StrategyConfig};
use crate::errors::TradingBotError;
use std::collections::HashMap;

pub struct BotStrategy;

impl BotStrategy {
    // Initialize a new bot strategy
    pub fn initialize(
        ctx: Context<InitializeBot>,
        config: StrategyConfig,
        strategy_id: String,
    ) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;
        strategy.owner = ctx.accounts.owner.key();
        strategy.strategy_id = strategy_id;
        strategy.config = config;
        strategy.is_active = false;
        strategy.created_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    // Execute strategy based on frontend configuration
    pub fn execute_strategy(
        ctx: Context<ExecuteStrategy>,
        blocks: Vec<StrategyBlock>,
    ) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;
        require!(strategy.is_active, TradingBotError::StrategyInactive);

        // Execute each block in the strategy
        for block in blocks {
            match block.block_type {
                BlockType::Trigger => {
                    Self::execute_trigger(ctx.accounts.clone(), &block)?;
                },
                BlockType::Action => {
                    Self::execute_action(ctx.accounts.clone(), &block)?;
                },
                BlockType::Condition => {
                    Self::execute_condition(ctx.accounts.clone(), &block)?;
                },
            }
        }

        Ok(())
    }

    // Execute trigger block (e.g., price conditions)
    fn execute_trigger(
        accounts: ExecuteStrategy,
        block: &StrategyBlock,
    ) -> Result<()> {
        match block.trigger_type {
            TriggerType::Price => {
                // Get price from Pyth oracle
                let price = PythOracle::get_price(
                    &accounts.price_feed,
                    60, // 60 seconds max staleness
                )?;

                // Check price condition
                Self::verify_price_condition(
                    price.price,
                    block.config.price_threshold,
                    block.config.condition_type,
                )?;
            },
            TriggerType::Volume => {
                // Implement volume trigger
            },
            TriggerType::Time => {
                // Implement time-based trigger
            },
        }

        Ok(())
    }

    // Execute action block (e.g., trades)
    fn execute_action(
        accounts: ExecuteStrategy,
        block: &StrategyBlock,
        state: &mut ExecutionState,
    ) -> Result<()> {
        match block.config.action_type {
            Some(ActionType::Swap) => {
                match block.config.parameters.dex_type {
                    Some(DexType::Raydium) => {
                        RaydiumDex::swap(
                            accounts.into(),
                            block.config.parameters.amount.unwrap(),
                            block.config.parameters.token_address.unwrap(),
                            block.config.parameters.slippage_bps.unwrap(),
                        )?;
                    },
                    Some(DexType::Jupiter) => {
                        JupiterDex::execute_swap(
                            accounts.into(),
                            block.config.parameters.amount.unwrap(),
                            block.config.parameters.token_address.unwrap(),
                            block.config.parameters.slippage_bps.unwrap(),
                        )?;
                    },
                    Some(DexType::Serum) => {
                        SerumDex::place_market_order(
                            accounts.into(),
                            block.config.parameters.amount.unwrap(),
                            block.config.parameters.token_address.unwrap(),
                        )?;
                    },
                    None => return Err(TradingBotError::InvalidDexType.into()),
                }
            },
            // Add other action types
            _ => return Err(TradingBotError::InvalidActionType.into()),
        }

        state.record_action_execution(block)?;
        Ok(())
    }

    // Execute condition block
    fn execute_condition(
        accounts: ExecuteStrategy,
        block: &StrategyBlock,
    ) -> Result<()> {
        match block.condition_type {
            ConditionType::Balance => {
                Self::verify_balance_condition(
                    &accounts.token_account,
                    block.config.minimum_balance,
                )?;
            },
            ConditionType::PriceImpact => {
                Self::verify_price_impact(
                    accounts.clone(),
                    block.config.max_price_impact,
                )?;
            },
            ConditionType::Custom => {
                // Implement custom conditions
            },
        }

        Ok(())
    }

    // Helper functions
    fn verify_price_condition(
        current_price: i64,
        threshold: i64,
        condition_type: PriceConditionType,
    ) -> Result<()> {
        match condition_type {
            PriceConditionType::Above => {
                require!(
                    current_price > threshold,
                    TradingBotError::ConditionNotMet
                );
            },
            PriceConditionType::Below => {
                require!(
                    current_price < threshold,
                    TradingBotError::ConditionNotMet
                );
            },
            PriceConditionType::Equal => {
                require!(
                    (current_price - threshold).abs() < 100, // Allow small deviation
                    TradingBotError::ConditionNotMet
                );
            },
        }

        Ok(())
    }

    // Execute strategy with block sequence
    pub fn execute_strategy_blocks(
        ctx: Context<ExecuteStrategy>,
        blocks: Vec<StrategyBlock>,
    ) -> Result<()> {
        let strategy = &mut ctx.accounts.strategy;
        require!(strategy.is_active, TradingBotError::StrategyInactive);

        // Track block execution state
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
}

// Add execution state tracking
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ExecutionState {
    pub executed_blocks: Vec<String>,
    pub loop_counters: HashMap<String, u64>,
    pub last_prices: HashMap<String, u64>,
    pub trade_results: Vec<TradeResult>,
}

impl ExecutionState {
    pub fn new() -> Self {
        Self {
            executed_blocks: Vec::new(),
            loop_counters: HashMap::new(),
            last_prices: HashMap::new(),
            trade_results: Vec::new(),
        }
    }

    pub fn record_action_execution(&mut self, block: &StrategyBlock) -> Result<()> {
        self.executed_blocks.push(block.id.clone());
        Ok(())
    }
}

// Account structures
#[derive(Accounts)]
pub struct InitializeBot<'info> {
    #[account(init, payer = owner, space = Strategy::LEN)]
    pub strategy: Account<'info, Strategy>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteStrategy<'info> {
    #[account(mut)]
    pub strategy: Account<'info, Strategy>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: Verified in program
    pub price_feed: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub owner: Signer<'info>,
}

// Strategy block types
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BlockType {
    Trigger,
    Action,
    Condition,
    Loop,
    Exit,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TriggerType {
    Price,
    Volume,
    Time,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ActionType {
    Swap,
    LiquidityProvision,
    Stake,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ConditionType {
    Balance,
    PriceImpact,
    Custom,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BlockConfig {
    pub amount: Option<u64>,
    pub minimum_out: Option<u64>,
    pub slippage_bps: Option<u16>,
    pub price_threshold: Option<i64>,
    pub condition_type: Option<PriceConditionType>,
    pub minimum_balance: Option<u64>,
    pub max_price_impact: Option<u16>,
    pub side: Option<Side>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PriceConditionType {
    Above,
    Below,
    Equal,
} 