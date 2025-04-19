use cosmwasm_schema::{cw_serde, QueryResponses};



#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    VerifyReceipt { receipt: String },
    StartGame { stake: Uint128 },
    JoinGame { game_id: String },
    SubmitMove { game_id: String, commitment: String },
    RevealMove { game_id: String, gesture: String, salt: String },
    EndGame { game_id: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GameStateResponse)]
    GetGameState { game_id: String },
}
