use cosmwasm_std::{entry_point, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

pub(crate) mod msg;
pub(crate) mod state;

mod contract;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}
