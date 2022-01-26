use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{collateral_manager::QueryStakeStatusMsg, ticket_manager::TicketResultMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub ticket_manager: Option<String>,
    pub collateral_manater: Option<String>,
    pub auction_manager: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    // Transactions initiated by user(worker)
    PlaceBet(PlaceBetMsg),
    LockStake { tid: u64 },
    SubmitResult(TicketResultMsg),

    // Utilities
    PostConfig(PostConfigMsg),
}

// Message for "PlaceBet" execute.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlaceBetMsg {
    pub ticket_id: u64,
    pub bet_amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PostConfigMsg {
    pub ticket_manager: Option<String>,
    pub collateral_manager: Option<String>,
    pub auction_manager: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    QueryBetAvailTickets {},
    QueryStakeStatus(QueryStakeStatusMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
