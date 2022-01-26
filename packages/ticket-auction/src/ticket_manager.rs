use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub auction_manager: Option<String>,
    pub user_board: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    // Transactions initiated by admin
    AddTicket(AddTicketMsg),
    UpdateTicket(UpdateTicketMsg),
    RemoveTicket { tid: u64 },
    SaveTicketWorker(TicketWorkerPair),
    AssessSubmission(TicketResultMsg),

    // Utilities
    PostConfig(PostConfigMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddTicketMsg {
    pub id: u64,
    pub bet_finish_timestamp: u64,
    pub close_timestamp: u64,
    pub result: String,
    pub collateral: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdateTicketMsg {
    pub id: u64,
    pub bet_finish_timestamp: Option<u64>,
    pub close_timestamp: Option<u64>,
    pub result: Option<String>,
    pub collateral: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TicketWorkerPair {
    pub tid: u64,
    pub worker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TicketResultMsg {
    pub tid: u64,
    pub worker: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PostConfigMsg {
    pub auction_manager: Option<String>,
    pub user_board: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    QueryTickets {},
    QueryTicketInfo { tid: u64 },
    QueryTicketWorkerPairs {},
    QueryTicketWorker { tid: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TicketInfoResponse {
    pub id: u64,
    pub bet_finish_timestamp: u64,
    pub close_timestamp: u64,
    pub result: String,
    pub collateral: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ticket {
    pub id: u64,
    pub bet_finish_timestamp: u64,
    pub close_timestamp: u64,
    pub result: String,
    pub collateral: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TicketsResponse {
    pub tickets: Vec<Ticket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TWPairsReponse {
    pub pairs: Vec<TicketWorkerPair>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
