pub mod jupiter;
pub mod raydium;

use anchor_lang::prelude::*;
// Common DEX traits and types
pub trait DexSwap {
    fn execute_swap(
        accounts: &dyn SwapAccounts,
        amount_in: u64,
        minimum_out: u64,
        slippage_bps: u16,
    ) -> Result<()>;
}

pub trait SwapAccounts {
    fn validate(&self) -> Result<()>;
}
