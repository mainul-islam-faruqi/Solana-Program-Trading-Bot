use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

// Official Raydium Program IDs from docs
pub const RAYDIUM_V3_PROGRAM_ID: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"; // CLMM Program
pub const RAYDIUM_AMM_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"; // OpenBook AMM
pub const RAYDIUM_ROUTER_PROGRAM_ID: &str = "routeUGWgWzqBWFcrCfv8tritsqukccJPu3q5GPP3xS"; // AMM Router

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    #[account(mut)]
    pub token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_out: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Raydium AMM Program
    #[account(address = RAYDIUM_AMM_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Pool state account
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    /// CHECK: Pool authority
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Pool open orders
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    pub owner: Signer<'info>,
}

impl<'info> RaydiumSwap<'info> {
    pub fn execute_swap(
        &self,
        amount_in: u64,
        minimum_out: u64,
        slippage_bps: u16,
    ) -> Result<()> {
        // Basic validation
        require!(amount_in > 0, TradingBotError::InvalidTradeConditions);
        require!(minimum_out > 0, TradingBotError::InvalidTradeConditions);
        require!(slippage_bps <= 10000, TradingBotError::InvalidTradeConditions);

        // Log swap details
        msg!("Executing Raydium swap");
        msg!("Amount in: {}", amount_in);
        msg!("Minimum out: {}", minimum_out);
        msg!("Slippage (bps): {}", slippage_bps);

        // Create swap instruction data
        let mut data = Vec::with_capacity(32);
        data.extend_from_slice(&[2]); // Instruction discriminator for swap
        data.extend_from_slice(&amount_in.to_le_bytes());
        data.extend_from_slice(&minimum_out.to_le_bytes());
        data.extend_from_slice(&slippage_bps.to_le_bytes());

        // Create CPI instruction for Raydium AMM
        let ix = solana_program::instruction::Instruction {
            program_id: *self.amm_program.key,
            accounts: vec![
                AccountMeta::new(*self.amm_id.key, false),
                AccountMeta::new(*self.amm_authority.key, false),
                AccountMeta::new(*self.amm_open_orders.key, false),
                AccountMeta::new(self.token_program.to_account_info().key(), false),
                AccountMeta::new(self.token_in.to_account_info().key(), true),
                AccountMeta::new(self.token_out.to_account_info().key(), true),
                AccountMeta::new(self.owner.key(), true),
            ],
            data,
        };

        // Execute the swap
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                self.amm_program.to_account_info(),
                self.amm_id.to_account_info(),
                self.amm_authority.to_account_info(),
                self.amm_open_orders.to_account_info(),
                self.token_program.to_account_info(),
                self.token_in.to_account_info(),
                self.token_out.to_account_info(),
                self.owner.to_account_info(),
            ],
        )?;

        msg!("Raydium swap executed successfully");
        Ok(())
    }
}

#[error_code]
pub enum TradingBotError {
    #[msg("Invalid trade conditions")]
    InvalidTradeConditions,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
    #[msg("Price out of range")]
    PriceOutOfRange,
} 