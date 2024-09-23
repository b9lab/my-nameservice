use cosmwasm_std::{Addr, Event, StdError, Storage};
use cw721::msg::OwnerOfResponse;
use cw_multi_test::{App, ContractWrapper, Executor};
use my_nameservice::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

fn instantiate_nameservice(mock_app: &mut App) -> (u64, Addr) {
    let nameservice_code = Box::new(ContractWrapper::new(execute, instantiate, query));
    let nameservice_code_id = mock_app.store_code(nameservice_code);
    return (
        nameservice_code_id,
        mock_app
            .instantiate_contract(
                nameservice_code_id,
                Addr::unchecked("deployer"),
                &InstantiateMsg {
                    name: "my names".to_owned(),
                    symbol: "MYN".to_owned(),
                    creator: None,
                    minter: Some("minter".to_owned()),
                    collection_info_extension: None,
                    withdraw_address: None,
                },
                &[],
                "nameservice",
                None,
            )
            .expect("Failed to instantiate nameservice"),
    );
}

#[test]
fn test_register() {
    // Arrange
    let mut mock_app = App::default();
    let (_, contract_addr) = instantiate_nameservice(&mut mock_app);
    let owner_addr_value = "owner".to_owned();
    let owner_addr = Addr::unchecked(owner_addr_value.clone());
    let name_alice = "alice".to_owned();
    let register_msg = ExecuteMsg::Mint {
        token_id: name_alice.to_owned(),
        owner: owner_addr.to_string(),
        extension: None,
        token_uri: None,
    };

    // Act
    let result = mock_app.execute_contract(
        Addr::unchecked("minter"),
        contract_addr.clone(),
        &register_msg,
        &[],
    );

    // Assert
    assert!(result.is_ok(), "Failed to register alice");
    let received_response = result.unwrap();
    let expected_event = Event::new("wasm")
        .add_attribute("_contract_address", "contract0".to_owned())
        .add_attribute("action", "mint".to_owned())
        .add_attribute("minter", "minter".to_owned())
        .add_attribute("owner", owner_addr_value.to_owned())
        .add_attribute("token_id", name_alice.to_owned());
    received_response.assert_event(&expected_event);
    assert_eq!(received_response.data, None);
    // Global storage
    let expected_key_main =
        format!("\0\u{4}wasm\0\u{17}contract_data/contract0\0\u{6}tokens{name_alice}",);
    let stored_addr_bytes = mock_app
        .storage()
        .get(expected_key_main.as_bytes())
        .expect("Failed to load from name alice");
    let stored_addr = String::from_utf8(stored_addr_bytes).unwrap();
    assert_eq!(
        stored_addr,
        format!(
            r#"{{"owner":"{owner_addr_value}","approvals":[],"token_uri":null,"extension":null}}"#
        )
    );
    // Storage local to contract
    let stored_addr_bytes = mock_app
        .contract_storage(&contract_addr)
        .get(format!("\0\u{6}tokens{name_alice}").as_bytes())
        .expect("Failed to load from name alice");
    let stored_addr = String::from_utf8(stored_addr_bytes).unwrap();
    assert_eq!(
        stored_addr,
        format!(
            r#"{{"owner":"{owner_addr_value}","approvals":[],"token_uri":null,"extension":null}}"#
        )
    );
}

#[test]
fn test_query() {
    // Arrange
    let mut mock_app = App::default();
    let (_, contract_addr) = instantiate_nameservice(&mut mock_app);
    let owner_addr = Addr::unchecked("owner");
    let name_alice = "alice".to_owned();
    let register_msg = ExecuteMsg::Mint {
        token_id: name_alice.to_owned(),
        owner: owner_addr.to_string(),
        extension: None,
        token_uri: None,
    };
    let _ = mock_app
        .execute_contract(
            Addr::unchecked("minter"),
            contract_addr.clone(),
            &register_msg,
            &[],
        )
        .expect("Failed to register alice");
    let resolve_record_query_msg = QueryMsg::OwnerOf {
        token_id: name_alice.to_owned(),
        include_expired: None,
    };

    // Act
    let result = mock_app
        .wrap()
        .query_wasm_smart::<OwnerOfResponse>(&contract_addr, &resolve_record_query_msg);

    // Assert
    assert!(result.is_ok(), "Failed to query alice name");
    assert_eq!(
        result.unwrap(),
        OwnerOfResponse {
            owner: owner_addr.to_string(),
            approvals: [].to_vec(),
        }
    )
}

#[test]
fn test_query_empty() {
    // Arrange
    let mut mock_app = App::default();
    let (_, contract_addr) = instantiate_nameservice(&mut mock_app);
    let name_alice = "alice".to_owned();
    let resolve_record_query_msg = QueryMsg::OwnerOf {
        token_id: name_alice.to_owned(),
        include_expired: None,
    };

    // Act
    let result = mock_app
        .wrap()
        .query_wasm_smart::<OwnerOfResponse>(&contract_addr, &resolve_record_query_msg);

    // Assert
    assert!(result.is_err(), "There was an unexpected value");
    assert_eq!(result.unwrap_err(), StdError::GenericErr { 
        msg: "Querier contract error: type: cw721::state::NftInfo<core::option::Option<cosmwasm_std::results::empty::Empty>>; key: [00, 06, 74, 6F, 6B, 65, 6E, 73, 61, 6C, 69, 63, 65] not found".to_owned(),
    });
}
