#![no_main]
#![no_std]

use risc0_zkvm_guest::env;
use sha2::{Sha256, Digest};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let gesture: u8 = env::read(); // Rock=0, Paper=1, Scissors=2
    let salt: [u8; 32] = env::read();
    let expected_commitment: [u8; 32] = env::read();

    let mut hasher = Sha256::new();
    hasher.update(&[gesture]);
    hasher.update(&salt);
    let actual_commitment = hasher.finalize();

    if actual_commitment[..] != expected_commitment[..] {
        panic!("Commitment mismatch");
    }

    env::commit(&gesture);
}