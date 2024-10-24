use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw721::msg::{Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721QueryMsg};

pub type InstantiateMsg = Cw721InstantiateMsg<Option<Empty>>;

pub type ExecuteMsg = Cw721ExecuteMsg<Option<Empty>, Option<Empty>, Empty>;

pub type QueryMsg = Cw721QueryMsg<Option<Empty>, Option<Empty>, Empty>;

#[cw_serde]
pub struct ExecuteMsgResponse {
    pub num_tokens: u64,
}
