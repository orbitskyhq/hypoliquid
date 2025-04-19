// zk_game_backend/server.rs
use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NewGameRequest {
    stake: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JoinGameRequest {
    game_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubmitMoveRequest {
    game_id: String,
    commitment: String,
    address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RevealMoveRequest {
    game_id: String,
    gesture: String,
    salt: String,
    address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GameState {
    players: Vec<String>,
    commitments: HashMap<String, String>,
    revealed: HashMap<String, String>,
    stake: u64,
}

static GAMES: Lazy<Arc<Mutex<HashMap<String, GameState>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/create_game", post(create_game))
        .route("/join_game", post(join_game))
        .route("/submit_move", post(submit_move))
        .route("/reveal_move", post(reveal_move))
        .route("/get_game_state", post(get_game_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_game(Json(payload): Json<NewGameRequest>) -> Json<String> {
    let game_id = format!("game-{}", uuid::Uuid::new_v4());
    let mut games = GAMES.lock().unwrap();
    games.insert(game_id.clone(), GameState {
        players: vec![],
        commitments: HashMap::new(),
        revealed: HashMap::new(),
        stake: payload.stake,
    });
    Json(game_id)
}

async fn join_game(Json(payload): Json<JoinGameRequest>) -> Json<&'static str> {
    let mut games = GAMES.lock().unwrap();
    if let Some(game) = games.get_mut(&payload.game_id) {
        game.players.push("playerX".to_string()); // hardcoded player stub
        Json("joined")
    } else {
        Json("game_not_found")
    }
}

async fn submit_move(Json(payload): Json<SubmitMoveRequest>) -> Json<&'static str> {
    let mut games = GAMES.lock().unwrap();
    if let Some(game) = games.get_mut(&payload.game_id) {
        game.commitments.insert(payload.address, payload.commitment);
        Json("move_submitted")
    } else {
        Json("game_not_found")
    }
}

async fn reveal_move(Json(payload): Json<RevealMoveRequest>) -> Json<&'static str> {
    let mut games = GAMES.lock().unwrap();
    if let Some(game) = games.get_mut(&payload.game_id) {
        game.revealed.insert(payload.address, payload.gesture);
        Json("move_revealed")
    } else {
        Json("game_not_found")
    }
}

async fn get_game_state(Json(payload): Json<JoinGameRequest>) -> Json<Option<GameState>> {
    let games = GAMES.lock().unwrap();
    Json(games.get(&payload.game_id).cloned())
}