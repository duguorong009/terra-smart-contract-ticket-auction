use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Order, StdResult, Storage};
use cw_storage_plus::{Item, Map};

use ticket_auction::{
    error::TAError,
    ticket_manager::{Ticket, TicketWorkerPair},
};

pub const TICKETS: Item<Vec<Ticket>> = Item::new("tickets");

// **=================================================
// ** Tickets: Read and write operations      ========
// **=================================================

// Store tickets
pub fn store_tickets(storage: &mut dyn Storage, tickets: Vec<Ticket>) -> StdResult<()> {
    TICKETS.save(storage, &tickets)
}

// Read tickets
pub fn read_tickets(storage: &dyn Storage) -> StdResult<Vec<Ticket>> {
    TICKETS.load(storage)
}

// Read a single ticket with given id
pub fn read_ticket_for_id(storage: &dyn Storage, tid: u64) -> StdResult<Ticket> {
    let tickets = TICKETS.load(storage)?;
    match tickets.into_iter().find(|t| t.id == tid) {
        Some(t) => Ok(t),
        None => Err(TAError::NotFound.into()),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_board: String,
    pub auction_manager: Option<String>,
    pub user_board: Option<String>,
}

pub const CONFIG: Item<Config> = Item::new("config");
// **=================================================
// ** Config: Read and write operations      ========
// **=================================================
// Store Config
pub fn store_config(storage: &mut dyn Storage, config: Config) -> StdResult<()> {
    CONFIG.save(storage, &config)
}

// Read Config
pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

pub const TWPAIR: Map<u64, String> = Map::new("TicketWorkerPair");
// **=================================================
// ** TWPAIR: Read and write operations       ========
// **=================================================
// Store pair
pub fn store_tw_pair(storage: &mut dyn Storage, msg: TicketWorkerPair) -> StdResult<()> {
    let key = msg.tid;
    let worker = msg.worker;
    TWPAIR.save(storage, key, &worker)
}

// Read pair
pub fn read_worker_for_ticket(storage: &dyn Storage, tid: u64) -> StdResult<String> {
    TWPAIR.load(storage, tid)
}

// Read all of tickets(array of won bots)
pub fn read_all_assigned_tickets(storage: &dyn Storage) -> StdResult<Vec<u64>> {
    let keys = TWPAIR
        .keys(storage, None, None, Order::Ascending)
        .map(|v| {
            v.unwrap()
        })
        .collect::<Vec<u64>>();
    Ok(keys)
}
