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

#[cfg(test)]
mod tests {
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{testing, Addr, Response};

    #[test]
    fn test_instantiate() {
        // Arrange
        let mut mocked_deps_mut = testing::mock_dependencies();
        let mocked_env = testing::mock_env();
        let mocked_addr = Addr::unchecked("addr");
        let mocked_msg_info = testing::message_info(&mocked_addr, &[]);

        let instantiate_msg = InstantiateMsg {};

        // Act
        let contract_result = super::instantiate(
            mocked_deps_mut.as_mut(),
            mocked_env,
            mocked_msg_info,
            instantiate_msg,
        );

        // Assert
        assert!(contract_result.is_ok(), "Failed to instantiate");
        assert_eq!(contract_result.unwrap(), Response::default())
    }
}
