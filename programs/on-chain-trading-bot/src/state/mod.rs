use anchor_lang::prelude::*;

#[account]
pub struct TradingState {
    pub owner: Pubkey,
    pub active_strategies: Vec<StrategyState>,
    pub total_value_locked: u64,
    pub performance_metrics: PerformanceMetrics,
    pub last_update: i64,
    pub risk_parameters: RiskParameters,
    pub trading_limits: TradingLimits,
}

#[account]
pub struct StrategyState {
    pub strategy_id: Pubkey,
    pub strategy_type: StrategyType,
    pub is_active: bool,
    pub total_trades: u64,
    pub profit_loss: i64,
    pub created_at: i64,
    pub last_trade_timestamp: i64,
    pub execution_metrics: ExecutionMetrics,
    pub risk_metrics: RiskMetrics,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PerformanceMetrics {
    pub total_profit_loss: i64,
    pub win_rate: u8,
    pub avg_return: i64,
    pub sharpe_ratio: i64,
    pub max_drawdown: u64,
    pub total_volume: u64,
    pub best_trade: i64,
    pub worst_trade: i64,
    pub avg_trade_duration: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ExecutionMetrics {
    pub successful_trades: u64,
    pub failed_trades: u64,
    pub avg_slippage: u16,
    pub total_gas_used: u64,
    pub avg_execution_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RiskMetrics {
    pub current_drawdown: u64,
    pub volatility: u64,
    pub var_95: u64, // 95% Value at Risk
    pub current_exposure: u64,
    pub risk_adjusted_return: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RiskParameters {
    pub max_trade_size: u64,
    pub max_daily_loss: u64,
    pub max_drawdown: u64,
    pub max_leverage: u8,
    pub min_profit_threshold: u64,
    pub max_slippage_tolerance: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TradingLimits {
    pub daily_volume_limit: u64,
    pub max_open_positions: u8,
    pub min_trade_interval: i64,
    pub max_gas_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum StrategyType {
    GridTrading,
    TrendFollowing,
    MeanReversion,
    Arbitrage,
    LiquidityProvision,
    Custom,
}

impl TradingState {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        4 + (32 * 10) + // active_strategies (max 10)
        8 + // total_value_locked
        32 + // performance_metrics
        8 + // last_update
        32 + // risk_parameters
        32; // trading_limits

    pub fn update_metrics(&mut self) -> Result<()> {
        self.last_update = Clock::get()?.unix_timestamp;
        
        // Calculate total value locked
        self.total_value_locked = self.active_strategies
            .iter()
            .map(|s| s.execution_metrics.total_gas_used)
            .sum();

        // Update performance metrics
        self.update_performance_metrics()?;

        Ok(())
    }

    fn update_performance_metrics(&mut self) -> Result<()> {
        let metrics = &mut self.performance_metrics;
        
        // Calculate total P&L
        metrics.total_profit_loss = self.active_strategies
            .iter()
            .map(|s| s.profit_loss)
            .sum();

        // Calculate win rate
        let total_trades: u64 = self.active_strategies
            .iter()
            .map(|s| s.total_trades)
            .sum();

        let winning_trades: u64 = self.active_strategies
            .iter()
            .map(|s| s.execution_metrics.successful_trades)
            .sum();

        metrics.win_rate = if total_trades > 0 {
            ((winning_trades * 100) / total_trades) as u8
        } else {
            0
        };

        // Update other metrics
        self.calculate_advanced_metrics()?;

        Ok(())
    }

    fn calculate_advanced_metrics(&mut self) -> Result<()> {
        // Calculate Sharpe ratio
        self.calculate_sharpe_ratio()?;

        // Calculate maximum drawdown
        self.calculate_max_drawdown()?;

        // Calculate other risk-adjusted metrics
        self.calculate_risk_metrics()?;

        Ok(())
    }

    fn calculate_sharpe_ratio(&mut self) -> Result<()> {
        // Implement Sharpe ratio calculation
        Ok(())
    }

    fn calculate_max_drawdown(&mut self) -> Result<()> {
        // Implement maximum drawdown calculation
        Ok(())
    }

    fn calculate_risk_metrics(&mut self) -> Result<()> {
        // Implement risk metrics calculation
        Ok(())
    }
}

impl StrategyState {
    pub fn update_execution_metrics(&mut self, trade_result: TradeResult) -> Result<()> {
        let metrics = &mut self.execution_metrics;

        if trade_result.success {
            metrics.successful_trades += 1;
        } else {
            metrics.failed_trades += 1;
        }

        // Update other metrics
        metrics.avg_slippage = self.calculate_avg_slippage(trade_result.slippage)?;
        metrics.total_gas_used += trade_result.gas_used;
        metrics.avg_execution_time = self.calculate_avg_execution_time(trade_result.execution_time)?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TradeResult {
    pub success: bool,
    pub profit_loss: i64,
    pub slippage: u16,
    pub gas_used: u64,
    pub execution_time: i64,
} 