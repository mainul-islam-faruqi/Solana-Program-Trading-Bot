use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::io::{self, Write};
use super::WritePubkey;

#[derive(Clone, Copy, Debug)]
pub struct PubkeyWrapper(pub Pubkey);

impl BorshSerialize for PubkeyWrapper {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_pubkey(&self.0)
    }
}

impl BorshDeserialize for PubkeyWrapper {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        if buf.len() < 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Insufficient length for Pubkey",
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&buf[..32]);
        *buf = &buf[32..];
        Ok(PubkeyWrapper(Pubkey::new_from_array(bytes)))
    }
}

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