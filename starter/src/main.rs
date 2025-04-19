mod server;

use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    // Pick two numbers
    let a: u64 = 7;
    let b: u64 = 191;

    // Multiply them inside the ZKP
    let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
    prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
    let receipt = prover.run().unwrap();

    let c: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    println!("I know the factors of {}, and I can prove it!", c);

    receipt.verify(MULTIPLY_ID).unwrap();

    // Run the HTTP server (must be async or inside tokio)
    // server::run(); ← only if server.rs exposes a sync function

    // If you're using Axum with async, do this instead:
    // use tokio::runtime::Runtime;
    // Runtime::new().unwrap().block_on(server::run());
}
