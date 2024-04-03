use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_board: String,
    pub ticket_manager: String,
    pub user_board: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub fn store_config(storage: &mut dyn Storage, config: Config) -> StdResult<()> {
    CONFIG.save(storage, &config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

pub const STAKES: Map<u64, Vec<Addr>> = Map::new("stakes");

pub fn store_stakes(storage: &mut dyn Storage, tid: u64, stakes: Vec<Addr>) -> StdResult<()> {
    STAKES.save(storage, tid, &stakes)
}

pub fn read_stakes(storage: &dyn Storage, tid: u64) -> StdResult<Vec<Addr>> {
    STAKES.load(storage, tid)
}
