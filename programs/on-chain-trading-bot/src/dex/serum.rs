use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use serum_dex::state::{Market, MarketState, OpenOrders};
use serum_dex::matching::{Side, OrderType};

pub struct SerumDex;

impl SerumDex {
    // Initialize user's OpenOrders account for Serum market
    pub fn initialize_open_orders(
        ctx: Context<InitializeOpenOrders>,
    ) -> Result<()> {
        // Create PDA for OpenOrders account
        let (open_orders_pda, _) = Pubkey::find_program_address(
            &[
                b"open-orders",
                ctx.accounts.market.key().as_ref(),
                ctx.accounts.owner.key().as_ref(),
            ],
            ctx.program_id,
        );

        // Initialize OpenOrders account
        serum_dex::instruction::init_open_orders(
            ctx.accounts.market.to_account_info(),
            ctx.accounts.open_orders.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.serum_program.to_account_info(),
        )?;

        Ok(())
    }

    // Place limit order on Serum orderbook
    pub fn place_limit_order(
        ctx: Context<SerumOrder>,
        side: Side,
        price: u64,
        size: u64,
    ) -> Result<()> {
        // Verify market state
        let market = Market::load(
            &ctx.accounts.market,
            ctx.accounts.serum_program.key,
        )?;

        // Place order
        serum_dex::instruction::new_order(
            market,
            ctx.accounts.open_orders.to_account_info(),
            ctx.accounts.request_queue.to_account_info(),
            ctx.accounts.event_queue.to_account_info(),
            ctx.accounts.bids.to_account_info(),
            ctx.accounts.asks.to_account_info(),
            ctx.accounts.base_vault.to_account_info(),
            ctx.accounts.quote_vault.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            side,
            price,
            size,
            OrderType::Limit,
        )?;

        Ok(())
    }

    // Cancel order
    pub fn cancel_order(
        ctx: Context<SerumOrder>,
        order_id: u128,
    ) -> Result<()> {
        let market = Market::load(
            &ctx.accounts.market,
            ctx.accounts.serum_program.key,
        )?;

        serum_dex::instruction::cancel_order(
            market,
            ctx.accounts.open_orders.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            Side::Buy,
            order_id,
        )?;

        Ok(())
    }

    // Settle funds after trades
    pub fn settle_funds(
        ctx: Context<SerumSettle>,
    ) -> Result<()> {
        let market = Market::load(
            &ctx.accounts.market,
            ctx.accounts.serum_program.key,
        )?;

        serum_dex::instruction::settle_funds(
            market,
            ctx.accounts.open_orders.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.base_vault.to_account_info(),
            ctx.accounts.quote_vault.to_account_info(),
            ctx.accounts.base_wallet.to_account_info(),
            ctx.accounts.quote_wallet.to_account_info(),
            ctx.accounts.vault_signer.to_account_info(),
        )?;

        Ok(())
    }

    // Match orders in orderbook
    pub fn match_orders(
        ctx: Context<SerumMatch>,
        limit: u16,
    ) -> Result<()> {
        let market = Market::load(
            &ctx.accounts.market,
            ctx.accounts.serum_program.key,
        )?;

        serum_dex::instruction::match_orders(
            market,
            ctx.accounts.request_queue.to_account_info(),
            ctx.accounts.event_queue.to_account_info(),
            ctx.accounts.bids.to_account_info(),
            ctx.accounts.asks.to_account_info(),
            limit,
        )?;

        Ok(())
    }

    // Market order execution
    pub fn place_market_order(
        ctx: Context<SerumOrder>,
        side: Side,
        size: u64,
    ) -> Result<()> {
        // Verify market state
        let market = Market::load(
            &ctx.accounts.market,
            ctx.accounts.serum_program.key,
        )?;

        // Place order
        serum_dex::instruction::new_order(
            market,
            ctx.accounts.open_orders.to_account_info(),
            ctx.accounts.request_queue.to_account_info(),
            ctx.accounts.event_queue.to_account_info(),
            ctx.accounts.bids.to_account_info(),
            ctx.accounts.asks.to_account_info(),
            ctx.accounts.base_vault.to_account_info(),
            ctx.accounts.quote_vault.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            side,
            0,
            size,
            OrderType::Market,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeOpenOrders<'info> {
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Verified in CPI
    pub serum_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SerumOrder<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub open_orders: Account<'info, OpenOrders>,
    #[account(mut)]
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub base_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    /// CHECK: Verified in CPI
    pub serum_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SerumSettle<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub open_orders: Account<'info, OpenOrders>,
    #[account(mut)]
    pub base_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub base_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_wallet: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    /// CHECK: Verified in CPI
    pub vault_signer: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    pub serum_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SerumMatch<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketState>,
    #[account(mut)]
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    /// CHECK: Verified in CPI
    pub serum_program: AccountInfo<'info>,
} 