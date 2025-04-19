use cw_storage_plus::Map;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub enum Gesture {
    Rock,
    Paper,
    Scissors,
}

#[cw_serde]
pub struct Game {
    pub id: String,
    pub players: Vec<Addr>,
    pub stake: u128,
    pub commitments: Vec<(Addr, String)>,
    pub revealed_moves: Vec<(Addr, Gesture)>,
    pub winner: Option<Addr>,
    pub active: bool,
}

pub const GAMES: Map<String, Game> = Map::new("games");