use cosmwasm_name_system_contract::{
    execute,
    execute::ExecuteMsg,
    instantiate, query,
    query::{AddrByNameResp, QueryMsg},
};
use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn test_addr_by_name() {
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
        Addr::unchecked("sender"),
        addr.clone(),
        &ExecuteMsg::RegisterName {
            name: "vitalik".into(),
        },
        &[],
    )
    .unwrap();

    let resp: AddrByNameResp = app
        .wrap()
        .query_wasm_smart(
            addr,
            &QueryMsg::AddrByName {
                name: "vitalik".into(),
            },
        )
        .unwrap();

    assert_eq!(Addr::unchecked("sender"), resp.addr);
}
