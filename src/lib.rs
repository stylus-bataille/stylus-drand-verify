//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

mod bls12_381;
mod drand_verify;
mod sha2;

use crate::drand_verify::{G2PubkeyRfc, Pubkey, G1};
use alloy_primitives::hex_literal::hex;
use stylus_sdk::abi::Bytes;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::prelude::*;

// Define some persistent storage using the Solidity ABI.
// `DrandVerify` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct DrandVerify {
    }
}

const PK: [u8; 96] = hex!("83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a");

/// Declare that `DrandVerify` is a contract with the following external methods.
#[external]
impl DrandVerify {
    /// Gets the number from storage.
    pub fn verify(&self, round_number: u64, sig: Bytes) -> bool {
        let pk = G2PubkeyRfc::from_fixed_unchecked(PK).expect("pk");
        pk.verify(round_number, &[], &sig).unwrap_or(false)
    }
}
// {"public_key":"83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a","period":3,"genesis_time":1692803367,"hash":"52db9ba70e0cc0f6eaf7803dd07447a1f5477735fd3f661792ba94600c84e971","groupHash":"f477d5c89f21a17c863a7f937c6a6d15859414d2be09cd448d4279af331c5d3e","schemeID":"bls-unchained-g1-rfc9380","metadata":{"beaconID":"quicknet"}}
//
// {"round":9367912,"randomness":"a646d1f17493408fdd89ee2ff71a0d1eab6087f23fdef7f60a2baec60436a496","signature":"845091b99635a8a5f93a15cd8622fae0c7ee5307fb5b888dc67ee4fe997b5b326dbd637bd7ab39e3188fd507ea94b314"}
