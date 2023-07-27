use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response, StdResult};

use crate::state::ADMIN;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}
