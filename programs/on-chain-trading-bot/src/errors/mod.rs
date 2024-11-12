use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowErrors {
    #[msg("DCA not closed")]
    DCANotClosed,
    #[msg("Unexpected balance")]
    UnexpectedBalance,
    #[msg("DCA not complete")]
    DCANotComplete,
    #[msg("Already airdropped")]
    Airdropped,
    #[msg("Unexpected airdrop amount")]
    UnexpectedAirdropAmount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
     #[msg("Overflow")]
    MathOverflow,
}


