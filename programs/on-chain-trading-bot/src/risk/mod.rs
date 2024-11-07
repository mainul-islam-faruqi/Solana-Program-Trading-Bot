use anchor_lang::prelude::*;
use crate::state::{TradingStrategy, RiskParameters};

pub struct RiskManager;

impl RiskManager {
    pub fn validate_trade(
        strategy: &TradingStrategy,
        trade_size: u64,
        current_price: u64,
    ) -> Result<bool> {
        let risk_params = &strategy.risk_parameters;

        // Check trade size
        if trade_size > risk_params.max_trade_size {
            return Ok(false);
        }

        // Check daily loss limit
        if strategy.performance_metrics.total_profit_loss < -(risk_params.daily_loss_limit as i64) {
            return Ok(false);
        }

        // Check position limit
        // Add more risk checks

        Ok(true)
    }

    pub fn update_metrics(
        strategy: &mut TradingStrategy,
        trade_result: i64,
    ) -> Result<()> {
        let metrics = &mut strategy.performance_metrics;
        metrics.total_profit_loss += trade_result;

        if trade_result > 0 {
            metrics.win_count += 1;
            metrics.largest_profit = metrics.largest_profit.max(trade_result as u64);
        } else {
            metrics.loss_count += 1;
            metrics.largest_loss = metrics.largest_loss.max((-trade_result) as u64);
        }

        Ok(())
    }
} 