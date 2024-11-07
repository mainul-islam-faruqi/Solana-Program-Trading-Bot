use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info, Price, PriceFeed};

pub struct RaydiumOracle;

impl RaydiumOracle {
    // Get price from Pyth oracle
    pub fn get_price(
        pyth_price_account: &AccountInfo,
        max_staleness: i64,
    ) -> Result<Price> {
        let price_feed: PriceFeed = load_price_feed_from_account_info(pyth_price_account)?;
        let current_timestamp = Clock::get()?.unix_timestamp;
        let price = price_feed.get_current_price()
            .ok_or(TradingBotError::PriceUnavailable)?;

        // Verify price staleness
        let last_update_timestamp = price_feed.get_current_price()
            .ok_or(TradingBotError::PriceUnavailable)?
            .publish_time;
        
        require!(
            current_timestamp - last_update_timestamp <= max_staleness,
            TradingBotError::StalePriceFeed
        );

        Ok(price)
    }

    // Get TWAP (Time-Weighted Average Price)
    pub fn get_twap(
        pyth_price_account: &AccountInfo,
        period: i64,
    ) -> Result<i64> {
        let price_feed: PriceFeed = load_price_feed_from_account_info(pyth_price_account)?;
        let current_timestamp = Clock::get()?.unix_timestamp;
        
        let mut sum_price = 0i128;
        let mut count = 0u64;
        
        // Calculate TWAP using price history
        for price_data in price_feed.iter_price_history() {
            if current_timestamp - price_data.publish_time <= period {
                sum_price += price_data.price as i128;
                count += 1;
            }
        }

        require!(count > 0, TradingBotError::InsufficientPriceData);
        
        Ok((sum_price / count as i128) as i64)
    }
} 