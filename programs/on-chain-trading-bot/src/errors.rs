use anchor_lang::prelude::*;

#[error_code]
pub enum TradingBotError {
    #[msg("Strategy is not active")]
    StrategyInactive,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid trade conditions")]
    InvalidTradeConditions,
    #[msg("Price feed is stale")]
    StalePriceFeed,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid market")]
    InvalidMarket,

    // DEX Integration Errors
    #[msg("Invalid market state")]
    InvalidMarketState,
    #[msg("Invalid order type")]
    InvalidOrderType,
    #[msg("Invalid route")]
    InvalidRoute,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid tick range")]
    InvalidTickRange,
    #[msg("Invalid staking pool")]
    InvalidStakingPool,
    #[msg("Invalid arbitrage route")]
    InvalidArbitrageRoute,
    #[msg("Insufficient profit")]
    InsufficientProfit,
    #[msg("Unsupported bridge")]
    UnsupportedBridge,
    #[msg("Missing price limit")]
    MissingPriceLimit,

    // Oracle Errors
    #[msg("Price feed is stale")]
    StalePriceFeed,
    #[msg("Price unavailable")]
    PriceUnavailable,
    #[msg("Invalid price data")]
    InvalidPriceData,

    // Account Errors
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Insufficient repayment")]
    InsufficientRepayment,

    // Calculation Errors
    #[msg("Calculation overflow")]
    Overflow,
    #[msg("Invalid calculation")]
    InvalidCalculation,
} 