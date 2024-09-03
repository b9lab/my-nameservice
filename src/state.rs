use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
}

pub const NAME_RESOLVER: Map<&[u8], NameRecord> = Map::new("name_resolver");
