use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub minter: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String, owner: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ResolveRecordResponse)]
    ResolveRecord { name: String },
}

#[cw_serde]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}
