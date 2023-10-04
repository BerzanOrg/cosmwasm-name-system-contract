use cosmwasm_name_system_contract::{execute, instantiate, query};
use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn test_instantiate() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &Empty {},
        &[],
        "Contract",
        None,
    )
    .unwrap();
}
