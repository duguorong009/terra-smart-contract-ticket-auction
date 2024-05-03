use cosmwasm_std::{Addr, Order, StdResult, Storage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_board: String,
    pub ticket_manager: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

// **=================================================
// ** Config: Read and write operations       ========
// **=================================================

// Save config
pub fn save_config(storage: &mut dyn Storage, config: Config) -> StdResult<()> {
    CONFIG.save(storage, &config)
}

// Read config
pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

// Bets storage
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BetDetail {
    pub worker: Addr,
    pub bet_amt: u64,
}

pub const BETS: Map<u64, Vec<BetDetail>> = Map::new("bets");

// **=================================================
// ** Bets: Read and write operations         ========
// **=================================================
pub fn save_bets_ticket(
    storage: &mut dyn Storage,
    tid: u64,
    bets: Vec<BetDetail>,
) -> StdResult<()> {
    BETS.save(storage, tid, &bets)
}

pub fn read_bets_ticket(storage: &dyn Storage, tid: u64) -> StdResult<Vec<BetDetail>> {
    BETS.load(storage, tid)
}

pub fn remove_bets_ticket(storage: &mut dyn Storage, tid: u64) -> StdResult<()> {
    BETS.remove(storage, tid);
    Ok(())
}

pub fn read_curr_avail_tickets(storage: &dyn Storage) -> StdResult<Vec<u64>> {
    let keys = BETS.keys(storage, None, None, Order::Ascending);
    let keys = keys.into_iter().map(|v| v.unwrap()).collect::<Vec<u64>>();
    Ok(keys)
}
