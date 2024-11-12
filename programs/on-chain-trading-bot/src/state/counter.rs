use anchor_lang::prelude::*;

#[account]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    pub const LEN: usize = 8 + 8; // discriminator + u64
} 