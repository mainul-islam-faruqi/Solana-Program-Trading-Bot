use anchor_lang::prelude::*;
use instructions::*;
pub mod math;

declare_id!("3seUuDx9nQXF18sEtcyZBkrf4YQjxHJuYFS26JVn1ERK");

pub mod constants;
pub mod instructions;
pub mod state;
pub mod errors;


#[program]
pub mod on_chain_trading_bot {
    use super::*;

    pub fn setup_dca(
        ctx: Context<SetupDca>,
        application_idx: u64,
        in_amount: u64,
        in_amount_per_cycle: u64,
        cycle_frequency: i64,
        min_out_amount: Option<u64>,
        max_out_amount: Option<u64>,
        start_at: Option<i64>,
    ) -> Result<()> {
        instructions::setup_dca(
            ctx,
            application_idx,
            in_amount,
            in_amount_per_cycle,
            cycle_frequency,
            min_out_amount,
            max_out_amount,
            start_at,
        )
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        instructions::close(ctx)
    }

    pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
        instructions::airdrop(ctx)
    }
}