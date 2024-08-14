use crate::{error::ContractError, msg::InstantiateMsg};
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};

type ContractResult = Result<Response, ContractError>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _: DepsMut,
    _: Env,
    _: MessageInfo,
    _: InstantiateMsg,
) -> ContractResult {
    Ok(Response::default())
}
