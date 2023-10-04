use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Event, MessageInfo, Response, StdError, StdResult};

use crate::state::NAMES;

#[cw_serde]
pub enum ExecuteMsg {
    RegisterName { name: String },
    UnregisterName { name: String },
}

pub fn register_name(deps: DepsMut, info: MessageInfo, name: String) -> StdResult<Response> {
    if NAMES.has(deps.storage, name.clone()) {
        Err(StdError::generic_err("name is already taken"))
    } else {
        let event = Event::new("registered_name")
            .add_attribute("addr", &info.sender)
            .add_attribute("name", &name);

        let resp = Response::new()
            .add_event(event)
            .add_attribute("action", "register_name");

        NAMES.save(deps.storage, name, &info.sender)?;

        Ok(resp)
    }
}

pub fn unregister_name(deps: DepsMut, info: MessageInfo, name: String) -> StdResult<Response> {
    if NAMES.has(deps.storage, name.clone()) {
        let event = Event::new("unregistered_name")
            .add_attribute("addr", &info.sender)
            .add_attribute("name", &name);

        let resp = Response::new()
            .add_event(event)
            .add_attribute("action", "unregister_name");

        let addr = NAMES.load(deps.storage, name.clone())?;

        if addr != info.sender {
            return Err(StdError::generic_err("sender is not authorized"));
        }

        NAMES.remove(deps.storage, name);

        Ok(resp)
    } else {
        Err(StdError::generic_err("name is already not taken"))
    }
}
