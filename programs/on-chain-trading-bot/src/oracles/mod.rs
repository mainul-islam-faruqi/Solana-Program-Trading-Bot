use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed};
use crate::errors::TradingBotError;

pub struct PythOracle;

#[derive(Accounts)]
pub struct SubscribePriceFeed<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = PriceSubscription::LEN,
        seeds = [
            b"price-subscription",
            feed_id.key().as_ref(),
            payer.key().as_ref()
        ],
        bump
    )]
    pub subscription: Account<'info, PriceSubscription>,
    /// CHECK: Verified in program
    pub feed_id: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefreshPriceData<'info> {
    #[account(
        mut,
        seeds = [
            b"price-subscription",
            subscription.feed_id.as_ref(),
            subscription.owner.as_ref()
        ],
        bump = subscription.bump
    )]
    pub subscription: Account<'info, PriceSubscription>,
    /// CHECK: Verified in program
    pub price_feed: AccountInfo<'info>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PriceSubscription {
    pub owner: Pubkey,
    pub feed_id: Pubkey,
    pub last_update: i64,
    pub update_interval: u64,
    pub last_price: i64,
    pub last_confidence: u64,
    pub confidence_interval: u64,
    pub is_active: bool,
    pub bump: u8,
}

impl PriceSubscription {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        32 + // feed_id
        8 + // last_update
        8 + // update_interval
        8 + // last_price
        8 + // last_confidence
        8 + // confidence_interval
        1 + // is_active
        1; // bump
}

impl PythOracle {
    // Subscribe to price updates
    pub fn subscribe_to_price_feed(
        ctx: Context<SubscribePriceFeed>,
        update_interval: u64,
        confidence_interval: u64,
    ) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        
        // Initialize subscription
        subscription.owner = ctx.accounts.payer.key();
        subscription.feed_id = ctx.accounts.feed_id.key();
        subscription.last_update = Clock::get()?.unix_timestamp;
        subscription.update_interval = update_interval;
        subscription.confidence_interval = confidence_interval;
        subscription.is_active = true;
        subscription.bump = *ctx.bumps.get("subscription").unwrap();

        // Verify initial price data
        let initial_price = Self::get_price_with_confidence(
            &ctx.accounts.feed_id,
            confidence_interval,
            60, // 60 seconds max staleness for initial price
        )?;

        subscription.last_price = initial_price.price;
        subscription.last_confidence = initial_price.confidence;

        Ok(())
    }

    // Refresh price data with validation
    pub fn refresh_price_data(
        ctx: Context<RefreshPriceData>,
    ) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let current_time = Clock::get()?.unix_timestamp;
        
        // Verify update interval
        require!(
            current_time - subscription.last_update >= subscription.update_interval as i64,
            TradingBotError::TooFrequentUpdates
        );

        // Get and validate new price
        let price_data = Self::get_price_with_confidence(
            &ctx.accounts.price_feed,
            subscription.confidence_interval,
            60, // 60 seconds max staleness
        )?;

        // Update subscription data
        subscription.last_price = price_data.price;
        subscription.last_confidence = price_data.confidence;
        subscription.last_update = current_time;
        
        Ok(())
    }

    // Get price with enhanced confidence validation
    pub fn get_price_with_confidence(
        price_feed_account: &AccountInfo,
        max_confidence_interval: u64,
        max_staleness: i64,
    ) -> Result<Price> {
        // Load price feed
        let price_feed: PriceFeed = load_price_feed_from_account_info(price_feed_account)?;
        let current_timestamp = Clock::get()?.unix_timestamp;
        
        // Get current price
        let price = price_feed.get_current_price()
            .ok_or(TradingBotError::PriceUnavailable)?;

        // Validate staleness
        require!(
            current_timestamp - price.publish_time <= max_staleness,
            TradingBotError::StalePriceFeed
        );

        // Enhanced confidence validations
        Self::validate_confidence_metrics(
            price.price,
            price.confidence,
            max_confidence_interval,
        )?;

        Ok(price)
    }

    // Validate confidence metrics
    fn validate_confidence_metrics(
        price: i64,
        confidence: u64,
        max_confidence_interval: u64,
    ) -> Result<()> {
        // Basic confidence check
        require!(
            confidence <= max_confidence_interval,
            TradingBotError::LowConfidence
        );

        // Relative confidence check (confidence should be within percentage of price)
        let relative_confidence = (confidence as f64 / price.abs() as f64) * 100.0;
        require!(
            relative_confidence <= 1.0, // 1% maximum relative confidence
            TradingBotError::ExcessiveConfidenceInterval
        );

        // Minimum confidence threshold
        require!(
            confidence >= 100, // Minimum confidence value
            TradingBotError::InsufficientConfidence
        );

        Ok(())
    }

    // Get exponential moving average price
    pub fn get_ema_price(
        price_feed: &AccountInfo,
        period: u64,
    ) -> Result<i64> {
        // Implementation as before
        Ok(0)
    }
}

// Add to errors.rs
#[error_code]
pub enum OracleError {
    #[msg("Price feed is stale")]
    StalePriceFeed,
    #[msg("Price confidence interval too high")]
    LowConfidence,
    #[msg("Insufficient price confidence")]
    InsufficientConfidence,
    #[msg("Excessive confidence interval")]
    ExcessiveConfidenceInterval,
    #[msg("Too frequent price updates")]
    TooFrequentUpdates,
    #[msg("Price unavailable")]
    PriceUnavailable,
} 