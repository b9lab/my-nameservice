use crate::msg::InstantiateMsg;
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _: DepsMut,
    _: Env,
    _: MessageInfo,
    _: InstantiateMsg,
) -> Result<Response, StdError> {
    Ok(Response::default())
}
