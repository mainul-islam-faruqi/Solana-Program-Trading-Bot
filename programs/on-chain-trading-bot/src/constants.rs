pub const PRICE_PRECISION: u64 = 1_000_000; // 6 decimals
pub const MAX_SLIPPAGE_BPS: u16 = 1000; // 10%
pub const MIN_TICK: i32 = -443636;
pub const MAX_TICK: i32 = 443636;
pub const TICK_SPACING: i32 = 1;
pub const MAX_ROUTES: u8 = 5;
pub const MIN_LIQUIDITY: u64 = 1000;
pub const MAX_DEADLINE: i64 = 3600; // 1 hour
pub const STALE_PRICE_THRESHOLD: i64 = 60; // 60 seconds 

pub const ESCROW_SEED: &[u8] = b"escrow";
pub const AIRDROP_BPS: u64 = 500; // 5%