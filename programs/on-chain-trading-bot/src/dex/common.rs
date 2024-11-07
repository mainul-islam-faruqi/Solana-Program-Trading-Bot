use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed};
use crate::errors::TradingBotError;

// Common interfaces for all DEXs
pub trait DexInterface {
    fn swap(&self, ctx: Context<DexSwap>, params: SwapParams) -> Result<()>;
    fn add_liquidity(&self, ctx: Context<DexLiquidity>, params: LiquidityParams) -> Result<()>;
    fn remove_liquidity(&self, ctx: Context<DexLiquidity>, amount: u64) -> Result<()>;
    fn get_price(&self, price_feed: &AccountInfo) -> Result<Price>;
}

// Common parameters for swaps
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapParams {
    pub amount_in: u64,
    pub minimum_out: u64,
    pub slippage_bps: u16,
    pub deadline: i64,
    pub route: Option<Vec<Pubkey>>,
}

// Common parameters for liquidity
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidityParams {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub min_lp_amount: u64,
    pub max_slippage_bps: u16,
}

// Common price feed interface
pub trait PriceFeedInterface {
    fn get_price(&self, max_staleness: i64) -> Result<Price>;
    fn get_twap(&self, period: i64) -> Result<i64>;
    fn verify_freshness(&self, max_staleness: i64) -> Result<bool>;
}

// Common account structures
#[derive(Accounts)]
pub struct DexSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_out: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DexLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// Common utilities for DEX operations
pub struct DexUtils;

impl DexUtils {
    // Calculate price impact
    pub fn calculate_price_impact(
        amount_in: u64,
        amount_out: u64,
        reserve_in: u64,
        reserve_out: u64,
    ) -> Result<u16> {
        let expected_out = amount_in
            .checked_mul(reserve_out)
            .ok_or(TradingBotError::Overflow)?
            .checked_div(reserve_in)
            .ok_or(TradingBotError::Overflow)?;

        let impact = expected_out
            .checked_sub(amount_out)
            .ok_or(TradingBotError::Overflow)?
            .checked_mul(10000)
            .ok_or(TradingBotError::Overflow)?
            .checked_div(expected_out)
            .ok_or(TradingBotError::Overflow)?;

        Ok(impact as u16)
    }

    // Validate slippage
    pub fn validate_slippage(
        actual_price: u64,
        expected_price: u64,
        max_slippage_bps: u16,
    ) -> Result<()> {
        let slippage = Self::calculate_price_impact(
            actual_price,
            expected_price,
            actual_price,
            expected_price,
        )?;

        require!(
            slippage <= max_slippage_bps,
            TradingBotError::SlippageExceeded
        );

        Ok(())
    }

    // Validate deadline
    pub fn validate_deadline(deadline: i64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time <= deadline,
            TradingBotError::DeadlineExceeded
        );
        Ok(())
    }

    // Calculate optimal swap amounts
    pub fn calculate_optimal_swap_amounts(
        amount_in: u64,
        reserve_in: u64,
        reserve_out: u64,
        fee_bps: u16,
    ) -> Result<u64> {
        let fee_multiplier = (10000 - fee_bps) as u64;
        let amount_with_fee = amount_in
            .checked_mul(fee_multiplier)
            .ok_or(TradingBotError::Overflow)?
            .checked_div(10000)
            .ok_or(TradingBotError::Overflow)?;

        let numerator = amount_with_fee
            .checked_mul(reserve_out)
            .ok_or(TradingBotError::Overflow)?;
        let denominator = reserve_in
            .checked_add(amount_with_fee)
            .ok_or(TradingBotError::Overflow)?;

        Ok(numerator.checked_div(denominator).ok_or(TradingBotError::Overflow)?)
    }

    // Verify price feed data
    pub fn verify_price_feed(
        price_feed: &AccountInfo,
        max_staleness: i64,
    ) -> Result<Price> {
        let price_feed: PriceFeed = load_price_feed_from_account_info(price_feed)?;
        let current_timestamp = Clock::get()?.unix_timestamp;
        
        let price = price_feed.get_current_price()
            .ok_or(TradingBotError::PriceUnavailable)?;

        let last_update = price.publish_time;
        require!(
            current_timestamp - last_update <= max_staleness,
            TradingBotError::StalePriceFeed
        );

        Ok(price)
    }

    // Calculate TWAP
    pub fn calculate_twap(
        price_feed: &AccountInfo,
        period: i64,
    ) -> Result<i64> {
        let price_feed: PriceFeed = load_price_feed_from_account_info(price_feed)?;
        let current_timestamp = Clock::get()?.unix_timestamp;
        
        let mut sum_price = 0i128;
        let mut count = 0u64;
        
        for price_data in price_feed.iter_price_history() {
            if current_timestamp - price_data.publish_time <= period {
                sum_price += price_data.price as i128;
                count += 1;
            }
        }

        require!(count > 0, TradingBotError::InsufficientPriceData);
        
        Ok((sum_price / count as i128) as i64)
    }

    // Transfer tokens safely
    pub fn transfer_tokens(
        from: AccountInfo,
        to: AccountInfo,
        authority: AccountInfo,
        amount: u64,
        token_program: AccountInfo,
    ) -> Result<()> {
        anchor_spl::token::transfer(
            CpiContext::new(
                token_program,
                Transfer {
                    from,
                    to,
                    authority,
                },
            ),
            amount,
        )
    }
}