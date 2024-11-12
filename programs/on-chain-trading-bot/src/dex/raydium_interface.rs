use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Clone)]
pub struct SwapInfo {
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub fee_rate: u16,
}

#[derive(Clone)]
pub struct MarketState {
    pub is_initialized: bool,
    pub pool_id: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
}

#[derive(Clone)]
pub struct Position {
    pub owner: Pubkey,
    pub liquidity: u64,
    pub lower_tick: i32,
    pub upper_tick: i32,
}

#[derive(Clone)]
pub struct StakingPool {
    pub pool_id: Pubkey,
    pub reward_token: Pubkey,
    pub total_staked: u64,
}

#[derive(Clone)]
pub struct UserStake {
    pub owner: Pubkey,
    pub amount: u64,
    pub rewards_owed: u64,
}

#[derive(Clone)]
pub struct UserRewardInfo {
    pub last_update_time: i64,
    pub reward_token: Pubkey,
    pub staked_amount: u64,
}

pub struct ConcentratedLiquidityPool {
    pub is_initialized: bool,
    pub current_tick: i32,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
}

impl ConcentratedLiquidityPool {
    pub fn get_current_tick(&self) -> Result<i32> {
        Ok(self.current_tick)
    }
} 