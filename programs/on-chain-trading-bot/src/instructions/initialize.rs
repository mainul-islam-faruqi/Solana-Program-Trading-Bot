use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = TradingStrategy::LEN)]
    pub strategy: Account<'info, TradingStrategy>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    strategy_id: String,
    config: StrategyConfig,
) -> Result<()> {
    let strategy = &mut ctx.accounts.strategy;
    strategy.owner = ctx.accounts.owner.key();
    strategy.strategy_id = strategy_id;
    strategy.config = config;
    strategy.is_active = false;
    strategy.total_trades = 0;
    strategy.created_at = Clock::get()?.unix_timestamp;
    Ok(())
} 