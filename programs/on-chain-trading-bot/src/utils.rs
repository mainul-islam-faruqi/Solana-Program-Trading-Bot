use anchor_lang::prelude::*;
use crate::{errors::TradingBotError, constants::*};

pub fn validate_slippage(slippage_bps: u16) -> Result<()> {
    require!(
        slippage_bps <= MAX_SLIPPAGE_BPS,
        TradingBotError::SlippageExceeded
    );
    Ok(())
}

pub fn validate_deadline(deadline: i64) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        deadline >= current_time && deadline <= current_time + MAX_DEADLINE,
        TradingBotError::InvalidCalculation
    );
    Ok(())
}

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

pub fn validate_tick_range(lower: i32, upper: i32) -> Result<()> {
    require!(
        lower >= MIN_TICK && upper <= MAX_TICK && lower < upper,
        TradingBotError::InvalidTickRange
    );
    require!(
        lower % TICK_SPACING == 0 && upper % TICK_SPACING == 0,
        TradingBotError::InvalidTickRange
    );
    Ok(())
} 