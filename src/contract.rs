use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse},
    state::{NameRecord, NAME_RESOLVER},
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response,
    StdResult,
};

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult {
    match msg {
        ExecuteMsg::Register { name } => execute_register(deps, info, name),
    }
}

fn execute_register(deps: DepsMut, info: MessageInfo, name: String) -> ContractResult {
    let key = name.as_bytes();
    let record = NameRecord {
        owner: info.sender.to_owned(),
    };

    if NAME_RESOLVER.has(deps.storage, key) {
        return Err(ContractError::NameTaken { name });
    }

    NAME_RESOLVER.save(deps.storage, key, &record)?;

    let registration_event = Event::new("name-register")
        .add_attribute("name", name)
        .add_attribute("owner", info.sender);
    let resp = Response::default().add_event(registration_event);
    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { name } => query_resolve_record(deps, name),
    }
}

fn query_resolve_record(deps: Deps, name: String) -> StdResult<Binary> {
    let key = name.as_bytes();

    let address = NAME_RESOLVER
        .may_load(deps.storage, key)?
        .map(|record| record.owner.to_string());

    let resp = ResolveRecordResponse { address };

    to_json_binary(&resp)
}

#[cfg(test)]
mod tests {
    use crate::{
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
        state::{NameRecord, NAME_RESOLVER},
    };
    use cosmwasm_std::{testing, Addr, Binary, Event, Response};

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

    #[test]
    fn test_execute() {
        // Arrange
        let mut mocked_deps_mut = testing::mock_dependencies();
        let mocked_env = testing::mock_env();
        let mocked_addr = Addr::unchecked("addr");
        let mocked_msg_info = testing::message_info(&mocked_addr, &[]);
        let name = "alice".to_owned();
        let execute_msg = ExecuteMsg::Register { name: name.clone() };

        // Act
        let contract_result = super::execute(
            mocked_deps_mut.as_mut(),
            mocked_env,
            mocked_msg_info,
            execute_msg,
        );

        // Assert
        assert!(contract_result.is_ok(), "Failed to register alice");
        let received_response = contract_result.unwrap();
        let expected_event = Event::new("name-register")
            .add_attribute("name", name.to_owned())
            .add_attribute("owner", mocked_addr.to_string());
        let expected_response = Response::default().add_event(expected_event);
        assert_eq!(received_response, expected_response);
        assert!(NAME_RESOLVER.has(mocked_deps_mut.as_ref().storage, name.as_bytes()));
        let stored = NAME_RESOLVER.load(mocked_deps_mut.as_ref().storage, name.as_bytes());
        assert!(stored.is_ok());
        assert_eq!(stored.unwrap(), NameRecord { owner: mocked_addr });
    }

    #[test]
    fn test_query() {
        // Arrange
        let mut mocked_deps_mut = testing::mock_dependencies();
        let mocked_env = testing::mock_env();
        let name = "alice".to_owned();
        let mocked_addr_value = "addr".to_owned();
        let mocked_addr = Addr::unchecked(mocked_addr_value.clone());
        let mocked_msg_info = testing::message_info(&mocked_addr, &[]);
        let _ = super::execute_register(mocked_deps_mut.as_mut(), mocked_msg_info, name.clone())
            .expect("Failed to register alice");
        let query_msg = QueryMsg::ResolveRecord { name };

        // Act
        let query_result = super::query(mocked_deps_mut.as_ref(), mocked_env, query_msg);

        // Assert
        assert!(query_result.is_ok(), "Failed to query alice name");
        let expected_response = format!(r#"{{"address":"{mocked_addr_value}"}}"#);
        let expected = Binary::new(expected_response.as_bytes().to_vec());
        assert_eq!(query_result.unwrap(), expected);
    }
}
