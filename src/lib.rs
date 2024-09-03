// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

mod bls12_381;
mod drand_verify;
mod sha2;

use crate::drand_verify::{G2PubkeyRfc, Pubkey};
use alloy_primitives::hex_literal::hex;
use stylus_sdk::abi::Bytes;

// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::prelude::*;

// Define some persistent storage using the Solidity ABI.
// `DrandVerify` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct DrandVerify {
    }
}

const PK: [u8; 96] = hex!("83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a");

/// DrandVerify validates a DRand beacon on-chain.
#[external]
impl DrandVerify {
    /// Return true if sig is the signature of the given round number under the Drand quicknet public key.
    pub fn verify(&self, round_number: u64, sig: Bytes) -> bool {
        let pk = G2PubkeyRfc::from_fixed_unchecked(PK).expect("pk");
        pk.verify(round_number, &[], &sig).unwrap_or(false)
    }
}
