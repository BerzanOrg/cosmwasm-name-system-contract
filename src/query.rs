use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Deps, StdResult};

use crate::state::NAMES;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AddrByNameResp)]
    AddrByName { name: String },
}

#[cw_serde]
pub struct AddrByNameResp {
    pub addr: Addr,
}

pub fn addr_by_name(deps: Deps, name: String) -> StdResult<AddrByNameResp> {
    let addr = NAMES.load(deps.storage, name)?;
    let resp = AddrByNameResp { addr };
    Ok(resp)
}
