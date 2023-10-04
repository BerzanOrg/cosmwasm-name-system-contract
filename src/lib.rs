pub mod execute;
pub mod query;
pub mod state;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: query::QueryMsg) -> StdResult<Binary> {
    match msg {
        query::QueryMsg::AddrByName { name } => to_binary(&query::addr_by_name(deps, name)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: execute::ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        execute::ExecuteMsg::RegisterName { name } => execute::register_name(deps, info, name),
        execute::ExecuteMsg::UnregisterName { name } => execute::unregister_name(deps, info, name),
    }
}
