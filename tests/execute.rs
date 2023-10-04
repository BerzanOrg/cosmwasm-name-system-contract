use cosmwasm_name_system_contract::{execute, execute::ExecuteMsg, instantiate, query};
use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn test_register_name() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &Empty {},
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::RegisterName {
                name: "vitalik".into(),
            },
            &[],
        )
        .unwrap();

    let event = resp
        .events
        .iter()
        .find(|ev| ev.ty == "wasm-registered_name")
        .unwrap();

    assert_eq!(
        event
            .attributes
            .iter()
            .find(|attr| attr.key == "addr")
            .unwrap()
            .value,
        "user"
    );

    assert_eq!(
        event
            .attributes
            .iter()
            .find(|attr| attr.key == "name")
            .unwrap()
            .value,
        "vitalik"
    );

    let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

    assert_eq!(
        wasm.attributes
            .iter()
            .find(|attr| attr.key == "action")
            .unwrap()
            .value,
        "register_name"
    );
}

#[test]
fn test_unregister_name() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &Empty {},
            &[],
            "Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::RegisterName {
            name: "vitalik".into(),
        },
        &[],
    )
    .unwrap();

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::UnregisterName {
                name: "vitalik".into(),
            },
            &[],
        )
        .unwrap();

    let event = resp
        .events
        .iter()
        .find(|ev| ev.ty == "wasm-unregistered_name")
        .unwrap();

    assert_eq!(
        event
            .attributes
            .iter()
            .find(|attr| attr.key == "addr")
            .unwrap()
            .value,
        "user"
    );

    assert_eq!(
        event
            .attributes
            .iter()
            .find(|attr| attr.key == "name")
            .unwrap()
            .value,
        "vitalik"
    );

    let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

    assert_eq!(
        wasm.attributes
            .iter()
            .find(|attr| attr.key == "action")
            .unwrap()
            .value,
        "unregister_name"
    );
}
