use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapRoute {
    pub token_in: Pubkey,
    pub token_out: Pubkey,
    pub amount: u64,
    pub minimum_out: u64,
    pub slippage_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PriceData {
    pub price: u64,
    pub confidence: u64,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PoolInfo {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_rate: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserPosition {
    pub owner: Pubkey,
    pub pool_id: Pubkey,
    pub liquidity: u64,
    pub lower_tick: i32,
    pub upper_tick: i32,
    pub rewards_owed: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TradeConfig {
    pub max_slippage: u16,
    pub deadline: i64,
    pub min_output: u64,
} 