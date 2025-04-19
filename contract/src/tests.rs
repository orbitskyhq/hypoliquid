use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use crate::msg::{ExectueMsg};
use crate::contract::*;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{GAMES, Game, Gesture};

use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};


#[test]
fn test_verify() {
    unimplemented!();
}

#[test]
fn test_start_game() {
    let mut app = App::default();
    let contract_id = app.store_code(contract());

    let creator = Addr::unchecked("creator");
    let msg = InstantiateMsg {};
    let contract_addr = app.instantiate_contract(contract_id, creator.clone(), &msg, &[], "RPS", None).unwrap();

    let start_msg = ExecuteMsg::StartGame { stake: Uint128::new(1230) };
    let _ = app.execute_contract(creator.clone(), contract_addr.clone(), &start_msg, &[]).unwrap();

    let game = GAMES.load(&app.wrap(), "game-1").unwrap();
    assert_eq!(game.stake, 1230);
    assert_eq!(game.players.len(), 1);
    assert!(game.active);
}

#[test]
fn test_join_game() {
    // Setup similar to above, but then execute JoinGame
    let mut app = App::default();
    let contract_id = app.store_code(contract());
    let creator = Addr::unchecked("creator");
    let player2 = Addr::unchecked("player2");

    let msg = InstantiateMsg {};
    let contract_addr = app.instantiate_contract(contract_id, creator.clone(), &msg, &[], "RPS", None).unwrap();

    let _ = app.execute_contract(creator.clone(), contract_addr.clone(), &ExecuteMsg::StartGame { stake: Uint128::new(1230) }, &[]).unwrap();

    let _ = app.execute_contract(player2.clone(), contract_addr.clone(), &ExecuteMsg::JoinGame { game_id: "game-1".to_string() }, &[]).unwrap();

    let game = GAMES.load(&app.wrap(), "game-1").unwrap();
    assert_eq!(game.players.len(), 2);
}

#[test]
fn test_submit_move() {
    let mut app = App::default();
    // Instantiate + start + join
    // Generate a commitment = hash(move + salt) off-chain
    let commitment = "some-hashed-string".to_string();

    let player = Addr::unchecked("player1");
    let _ = app.execute_contract(player.clone(), contract_addr.clone(), &ExecuteMsg::SubmitMove {
        game_id: "game-1".to_string(),
        commitment,
    }, &[]).unwrap();

    let game = GAMES.load(&app.wrap(), "game-1").unwrap();
    assert_eq!(game.commitments.len(), 1);
}

#[test]
fn test_reveal_move_and_verify() {
    use sha2::{Sha256, Digest};

    let mut app = App::default();
    let contract_id = app.store_code(contract());
    let creator = Addr::unchecked("player1");

    // Instantiate contract
    let msg = InstantiateMsg {};
    let contract_addr = app.instantiate_contract(contract_id, creator.clone(), &msg, &[], "RPS", None).unwrap();

    // Start a game
    let _ = app.execute_contract(creator.clone(), contract_addr.clone(), &ExecuteMsg::StartGame {
        stake: Uint128::new(1230),
    }, &[]).unwrap();

    // Create a gesture and salt
    let gesture: u8 = 1; // Paper
    let salt: [u8; 32] = [42u8; 32]; // Arbitrary salt

    // Hash(gesture || salt)
    let mut hasher = Sha256::new();
    hasher.update(&[gesture]);
    hasher.update(&salt);
    let commitment_hash = hasher.finalize();
    let commitment_hex = hex::encode(commitment_hash);

    // Submit the commitment
    let _ = app.execute_contract(creator.clone(), contract_addr.clone(), &ExecuteMsg::SubmitMove {
        game_id: "game-1".to_string(),
        commitment: commitment_hex.clone(),
    }, &[]).unwrap();

    // Normally you'd generate a proof here using risc0 zkVM, but we'll mock it
    use risc0_zkvm::{serde::to_vec, host::Prover};
    use methods::{RPS_ID, RPS_PATH};

    let mut prover = Prover::new(&std::fs::read(RPS_PATH).unwrap(), RPS_ID).unwrap();
    prover.add_input(to_vec(&gesture).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&salt).unwrap().as_slice()).unwrap();
    prover.add_input(&commitment_hash).unwrap();

    let receipt = prover.run().unwrap();
    let encoded_receipt = base64::encode(bincode::serialize(&receipt).unwrap());

    // Verify receipt on-chain
    let _ = app.execute_contract(creator.clone(), contract_addr.clone(), &ExecuteMsg::VerifyReceipt {
        receipt: encoded_receipt,
    }, &[]).unwrap();

    // Now reveal the move
    let _ = app.execute_contract(creator.clone(), contract_addr.clone(), &ExecuteMsg::RevealMove {
        game_id: "game-1".to_string(),
        gesture: "Paper".to_string(),
        salt: hex::encode(salt),
    }, &[]).unwrap();

    // Optionally: assert game state was updated
    let game = GAMES.load(&app.wrap(), "game-1").unwrap();
    assert_eq!(game.revealed_moves.len(), 1);
}


#[test]
fn test_multiple_games_concurrently() {
    let mut app = App::default();
    let contract_id = app.store_code(contract());
    let msg = InstantiateMsg {};
    let creator1 = Addr::unchecked("p1");
    let creator2 = Addr::unchecked("p2");

    let contract_addr = app.instantiate_contract(contract_id, creator1.clone(), &msg, &[], "RPS", None).unwrap();

    let _ = app.execute_contract(creator1.clone(), contract_addr.clone(), &ExecuteMsg::StartGame { stake: Uint128::new(100) }, &[]).unwrap();
    let _ = app.execute_contract(creator2.clone(), contract_addr.clone(), &ExecuteMsg::StartGame { stake: Uint128::new(100) }, &[]).unwrap();

    let g1 = GAMES.load(&app.wrap(), "game-1").unwrap();
    let g2 = GAMES.load(&app.wrap(), "game-2").unwrap();

    assert_ne!(g1.id, g2.id);
    assert_eq!(g1.players.len(), 1);
    assert_eq!(g2.players.len(), 1);
}

fn contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

