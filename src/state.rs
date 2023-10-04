use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const NAMES: Map<String, Addr> = Map::new("names");
