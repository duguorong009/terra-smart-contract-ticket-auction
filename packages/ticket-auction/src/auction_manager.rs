use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub ticket_manager: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    // Transaction initiated by user_board
    PlaceBet(PlaceBetMsg),

    // Transaction initiated by admin_board
    DecideWinningBet { tid: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    BetAvailableTickets,
    CurrActiveBets { tid: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

// Message for "PlaceBet" execute.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlaceBetMsg {
    pub ticket_id: u64,
    pub workder: String,
    pub bet_amount: u64,
}
