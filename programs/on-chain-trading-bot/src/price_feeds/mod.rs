use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;

pub struct PriceFeeds;

impl PriceFeeds {
    pub fn get_price(
        pyth_price_account: &AccountInfo,
    ) -> Result<i64> {
        let price_feed = load_price_feed_from_account_info(pyth_price_account)?;
        let current_price = price_feed.get_current_price()?;
        
        Ok(current_price.price)
    }

    pub fn validate_price_freshness(
        pyth_price_account: &AccountInfo,
        max_staleness: i64,
    ) -> Result<bool> {
        let price_feed = load_price_feed_from_account_info(pyth_price_account)?;
        let current_time = Clock::get()?.unix_timestamp;
        let price_timestamp = price_feed.get_current_price()?.publish_time;
        
        Ok(current_time - price_timestamp <= max_staleness)
    }
} 