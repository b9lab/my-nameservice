use cosmwasm_std::{Addr, Api, CanonicalAddr, Event};
use cw_multi_test::{App, ContractWrapper, Executor};
use my_nameservice::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse},
};

type ContractAddr = Addr;
type MinterAddr = Addr;

fn instantiate_nameservice(mock_app: &mut App) -> (u64, ContractAddr, MinterAddr) {
    let nameservice_code = Box::new(ContractWrapper::new(execute, instantiate, query));
    let nameservice_code_id = mock_app.store_code(nameservice_code);
    let minter = mock_app
        .api()
        .addr_humanize(&CanonicalAddr::from("minter".as_bytes()))
        .unwrap();
    return (
        nameservice_code_id,
        mock_app
            .instantiate_contract(
                nameservice_code_id,
                Addr::unchecked("deployer"),
                &InstantiateMsg {
                    minter: minter.to_string(),
                },
                &[],
                "nameservice",
                None,
            )
            .expect("Failed to instantiate nameservice"),
        minter,
    );
}

#[test]
fn test_register() {
    // Arrange
    let mut mock_app = App::default();
    let (_, contract_addr, minter) = instantiate_nameservice(&mut mock_app);
    let owner_addr_value = "owner".to_owned();
    let owner_addr = Addr::unchecked(owner_addr_value.clone());
    let name_alice = "alice".to_owned();
    let register_msg = ExecuteMsg::Register {
        name: name_alice.to_owned(),
        owner: owner_addr.to_owned(),
    };

    // Act
    let result = mock_app.execute_contract(
        minter,
        contract_addr.clone(),
        &register_msg,
        &[],
    );

    // Assert
    assert!(result.is_ok(), "Failed to register alice");
    let received_response = result.unwrap();
    let expected_event = Event::new("wasm-name-register")
        .add_attribute("name", name_alice.to_owned())
        .add_attribute("owner", owner_addr_value.to_owned());
    received_response.assert_event(&expected_event);
    assert_eq!(received_response.data, None);
    let stored_addr_bytes = mock_app
        .contract_storage(&contract_addr)
        .get(format!("\0\rname_resolver{name_alice}").as_bytes())
        .expect("Failed to load from name alice");
    let stored_addr = String::from_utf8(stored_addr_bytes).unwrap();
    assert_eq!(stored_addr, format!(r#"{{"owner":"{owner_addr_value}"}}"#));
}

#[test]
fn test_query() {
    // Arrange
    let mut mock_app = App::default();
    let (_, contract_addr, minter) = instantiate_nameservice(&mut mock_app);
    let owner_addr = Addr::unchecked("owner");
    let name_alice = "alice".to_owned();
    let register_msg = ExecuteMsg::Register {
        name: name_alice.to_owned(),
        owner: owner_addr.to_owned(),
    };
    let _ = mock_app
        .execute_contract(
            minter,
            contract_addr.clone(),
            &register_msg,
            &[],
        )
        .expect("Failed to register alice");
    let resolve_record_query_msg = QueryMsg::ResolveRecord {
        name: name_alice.to_owned(),
    };

    // Act
    let result = mock_app
        .wrap()
        .query_wasm_smart::<ResolveRecordResponse>(&contract_addr, &resolve_record_query_msg);

    // Assert
    assert!(result.is_ok(), "Failed to query alice name");
    assert_eq!(
        result.unwrap(),
        ResolveRecordResponse {
            address: Some(owner_addr.to_string())
        }
    )
}

#[test]
fn test_query_empty() {
    // Arrange
    let mut mock_app = App::default();
    let (_, contract_addr, _) = instantiate_nameservice(&mut mock_app);
    let name_alice = "alice".to_owned();
    let resolve_record_query_msg = QueryMsg::ResolveRecord {
        name: name_alice.to_owned(),
    };

    // Act
    let result = mock_app
        .wrap()
        .query_wasm_smart::<ResolveRecordResponse>(&contract_addr, &resolve_record_query_msg);

    // Assert
    assert!(result.is_ok(), "Failed to query alice name");
    assert_eq!(result.unwrap(), ResolveRecordResponse { address: None })
}
