use anchor_lang::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct PubkeyWrapper(pub Pubkey);

impl From<Pubkey> for PubkeyWrapper {
    fn from(pubkey: Pubkey) -> Self {
        PubkeyWrapper(pubkey)
    }
}

impl From<PubkeyWrapper> for Pubkey {
    fn from(wrapper: PubkeyWrapper) -> Self {
        wrapper.0
    }
} 