#![allow(dead_code)]

use fastcrypto::secp256k1;

mod checksum;
mod pubkey;

fn main() {
    #[cfg(feature = "checksum")]
    {
        checksum::checksum();
    }

    #[cfg(feature = "pubkey")]
    {
        pubkey::pubkeymine();
    }
}
