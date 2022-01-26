use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ticket_manager::{AddTicketMsg, UpdateTicketMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    // Transactions initiated by admin wallet
    AddTicket(AddTicketMsg),
    UpdateTicket(UpdateTicketMsg),
    RemoveTicket { tid: u64 },
    DecideWinningBet { tid: u64 },

    // Transaction initiated by ticket_manager
    ReleaseStakeWithSlash(SlashMsg),

    // Utilities
    CreateTicketManager { code_id: u64 },
    CreateUsrBoardManager { code_id: u64 },
    CreateAuctionManager { code_id: u64 },
    CreateCollateralManager { code_id: u64 },

    PostConfig(PostConfigMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SlashMsg {
    pub tid: u64,
    pub worker: Addr,
    pub slash_perc: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    QueryTicketInfo { tid: u64 },
    QueryTicketWorker { tid: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PostConfigMsg {
    pub ticket_manager: Option<String>,
    pub collateral_manager: Option<String>,
    pub auction_manager: Option<String>,
    pub user_board: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
