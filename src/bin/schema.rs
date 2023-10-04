use cosmwasm_name_system_contract::execute::ExecuteMsg;
use cosmwasm_name_system_contract::query::QueryMsg;

use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

fn main() {
    write_api! {
        instantiate: Empty,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
