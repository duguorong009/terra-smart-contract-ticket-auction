use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_board: String,
    pub ticket_manager: Option<String>,
    pub collateral_manager: Option<String>,
    pub auction_manager: Option<String>,
}

pub const CONFIG: Item<Config> = Item::new("Config");
// **=================================================
// ** Config: Read and write operations       ========
// **=================================================
// Store Config
pub fn store_config(storage: &mut dyn Storage, config: Config) -> StdResult<()> {
    CONFIG.save(storage, &config)
}

// Read Config
pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}
