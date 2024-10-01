use crate::{
    error::ContractError,
    msg::{ExecuteMsg, ExecuteMsgResponse, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
};
use cw721::{
    extension::Cw721EmptyExtensions,
    traits::{Cw721Execute, Cw721Query},
};

type ContractResult = Result<Response, ContractError>;
type BinaryResult = Result<Binary, ContractError>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult {
    Ok(Cw721EmptyExtensions::default().instantiate(deps, &env, &info, msg)?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult {
    let library = Cw721EmptyExtensions::default();
    Ok(library
        .execute(deps.branch(), &env, &info, msg)
        .inspect(|response| assert_eq!(response.data, None))?
        .set_data(to_json_binary(&ExecuteMsgResponse {
            num_tokens: library.query_num_tokens(deps.storage)?.count,
        })?))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> BinaryResult {
    Ok(Cw721EmptyExtensions::default().query(deps, &env, msg)?)
}

#[cfg(test)]
mod tests {
    use crate::msg::{ExecuteMsg, ExecuteMsgResponse, InstantiateMsg, QueryMsg};
    use cosmwasm_std::{testing, to_json_binary, Addr, Binary, Response};
    use cw721::{
        extension::Cw721EmptyExtensions,
        state::{NftInfo, MINTER},
    };

    fn simple_instantiate_msg(minter: String) -> InstantiateMsg {
        InstantiateMsg {
            name: "my names".to_owned(),
            symbol: "MYN".to_owned(),
            creator: None,
            minter: Some(minter.to_string()),
            collection_info_extension: None,
            withdraw_address: None,
        }
    }

    #[test]
    fn test_instantiate() {
        // Arrange
        let mut mocked_deps_mut = testing::mock_dependencies();
        let mocked_env = testing::mock_env();
        let mocked_addr = Addr::unchecked("addr");
        let mocked_msg_info = testing::mock_info(&mocked_addr.to_string(), &[]);
        let minter = Addr::unchecked("minter");
        let instantiate_msg = simple_instantiate_msg(minter.to_string());

        // Act
        let contract_result = super::instantiate(
            mocked_deps_mut.as_mut(),
            mocked_env,
            mocked_msg_info,
            instantiate_msg,
        );

        // Assert
        assert!(contract_result.is_ok(), "Failed to instantiate");
        assert_eq!(
            contract_result.unwrap(),
            Response::default()
                .add_attribute("minter", "minter")
                .add_attribute("creator", "addr")
        );
        assert!(MINTER
            .assert_owner(&mocked_deps_mut.storage, &minter)
            .is_ok());
    }

    #[test]
    fn test_execute() {
        // Arrange
        let mut mocked_deps_mut = testing::mock_dependencies();
        let mocked_env = testing::mock_env();
        let mocked_addr = Addr::unchecked("addr");
        let minter = Addr::unchecked("minter");
        let _ = super::instantiate(
            mocked_deps_mut.as_mut(),
            mocked_env.to_owned(),
            testing::mock_info(&mocked_addr.to_string(), &[]),
            simple_instantiate_msg(minter.to_string()),
        )
        .expect("Failed to instantiate");
        let mocked_msg_info = testing::mock_info(&minter.to_string(), &[]);
        let name = "alice".to_owned();
        let owner = Addr::unchecked("owner");
        let execute_msg = ExecuteMsg::Mint {
            token_id: name.to_owned(),
            owner: owner.to_string(),
            token_uri: None,
            extension: None,
        };

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
        let expected_response = Response::default()
            .set_data(
                to_json_binary(&ExecuteMsgResponse { num_tokens: 1 })
                    .expect("Failed to serialize counter"),
            )
            .add_attribute("action", "mint")
            .add_attribute("minter", "minter")
            .add_attribute("owner", "owner")
            .add_attribute("token_id", "alice");
        assert_eq!(received_response, expected_response);
        assert!(Cw721EmptyExtensions::default()
            .config
            .nft_info
            .has(mocked_deps_mut.as_ref().storage, name.as_str()));
        let stored = Cw721EmptyExtensions::default()
            .config
            .nft_info
            .load(mocked_deps_mut.as_ref().storage, name.as_str());
        assert!(stored.is_ok());
        assert_eq!(
            stored.unwrap(),
            NftInfo {
                owner: owner,
                approvals: [].to_vec(),
                token_uri: None,
                extension: None,
            }
        );
    }

    #[test]
    fn test_query() {
        // Arrange
        let mut mocked_deps_mut = testing::mock_dependencies();
        let mocked_env = testing::mock_env();
        let name = "alice".to_owned();
        let mocked_addr_value = "addr".to_owned();
        let mocked_addr = Addr::unchecked(mocked_addr_value.clone());
        let minter = Addr::unchecked("minter");
        let _ = super::instantiate(
            mocked_deps_mut.as_mut(),
            mocked_env.to_owned(),
            testing::mock_info(&mocked_addr.to_string(), &[]),
            simple_instantiate_msg(minter.to_string()),
        )
        .expect("Failed to instantiate");
        let mocked_msg_info = testing::mock_info(&minter.to_string(), &[]);
        let execute_msg = ExecuteMsg::Mint {
            token_id: name.to_owned(),
            owner: mocked_addr.to_string(),
            token_uri: None,
            extension: None,
        };
        let _ = super::execute(
            mocked_deps_mut.as_mut(),
            mocked_env.to_owned(),
            mocked_msg_info,
            execute_msg,
        )
        .expect("Failed to register alice");
        let query_msg = QueryMsg::OwnerOf {
            token_id: name,
            include_expired: None,
        };

        // Act
        let query_result = super::query(mocked_deps_mut.as_ref(), mocked_env, query_msg);

        // Assert
        assert!(query_result.is_ok(), "Failed to query alice name");
        let expected_response = format!(r#"{{"owner":"{mocked_addr_value}","approvals":[]}}"#);
        let expected = Binary::from(expected_response.as_bytes());
        assert_eq!(query_result.unwrap(), expected);
    }
}
