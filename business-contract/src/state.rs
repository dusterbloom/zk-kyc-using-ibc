use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub struct Wlmap {}

pub const WHITELIST_MAP: Map<String, bool> = Map::new("whitelist_map");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ian {
    pub ian: String,
    pub id: u32,
    pub owner_chain: String,
    pub owner_address: String,
    pub application_chain: String,
    pub application_address: String,
    pub settlement_chain: String,
    pub settlement_address: String,
    pub private: bool,
}
// Application name as string
pub const IANS: Map<'_, String, Ian> = Map::new("ian_record");
pub const IANS_SEQ: Item<'_, u32> = Item::new("ian_record_seq");
