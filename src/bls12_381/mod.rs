//! # `bls12_381`
//!
//! This crate provides an implementation of the BLS12-381 pairing-friendly elliptic
//! curve construction.
//!
//! * **This implementation has not been reviewed or audited. Use at your own risk.**
//! * This implementation targets Rust `1.36` or later.
//! * This implementation does not require the Rust standard library.
//! * All operations are constant time unless explicitly noted.

#[cfg(test)]
mod tests;

#[macro_use]
mod util;

/// Notes about how the BLS12-381 elliptic curve is designed, specified
/// and implemented by this library.
pub mod notes {
    pub mod design;
    pub mod serialization;
}

mod scalar;

pub use scalar::Scalar;

mod fp;
mod fp2;
mod g1;
mod g2;

pub use g1::{G1Affine, G1Projective};
pub use g2::{G2Affine, G2Projective};

mod fp12;
mod fp6;

// The BLS parameter x for BLS12-381 is -0xd201000000010000
const BLS_X: u64 = 0xd201_0000_0001_0000;
const BLS_X_IS_NEGATIVE: bool = true;

mod pairings;

pub use pairings::{pairing, Bls12, Gt, MillerLoopResult};

pub use pairings::{multi_miller_loop, G2Prepared};

/// Use the generic_array re-exported by digest to avoid a version mismatch
pub(crate) use digest::generic_array;

pub mod hash_to_curve;
