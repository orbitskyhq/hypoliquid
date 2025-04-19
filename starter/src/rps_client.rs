// starter/src/rps_client.rs
use methods::{RPS_ID, RPS_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{to_vec};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use base64;

fn main() {
    println!("Choose your gesture (0 = Rock, 1 = Paper, 2 = Scissors):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let gesture: u8 = input.trim().parse().expect("Invalid input");

    let salt: [u8; 32] = [42u8; 32]; // could be randomized in production

    // Generate commitment hash
    let mut hasher = Sha256::new();
    hasher.update(&[gesture]);
    hasher.update(&salt);
    let commitment = hasher.finalize();

    println!("Commitment (hex): {}", hex::encode(&commitment));

    // Run RISC Zero prover
    let method_code = fs::read(RPS_PATH).expect("Failed to read method ELF");
    let mut prover = Prover::new(&method_code, RPS_ID).unwrap();

    prover.add_input(to_vec(&gesture).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&salt).unwrap().as_slice()).unwrap();
    prover.add_input(&commitment).unwrap();

    let receipt = prover.run().unwrap();
    receipt.verify(RPS_ID).unwrap();

    let encoded_receipt = base64::encode(bincode::serialize(&receipt).unwrap());

    println!("Base64-encoded receipt:");
    println!("{}", encoded_receipt);

    // Simulate HTTP POST to CosmWasm backend (mock or CLI-based for now)
    println!("Send this receipt to your CosmWasm contract with ExecuteMsg::VerifyReceipt");
}