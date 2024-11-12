pub mod pubkey_ext;

use anchor_lang::prelude::*;
use std::io::{self, Write};

pub trait WritePubkey {
    fn write_pubkey(&mut self, pubkey: &Pubkey) -> io::Result<()>;
}

impl<W: Write> WritePubkey for W {
    fn write_pubkey(&mut self, pubkey: &Pubkey) -> io::Result<()> {
        self.write_all(pubkey.as_ref())
    }
} 