use cosmwasm_std::{Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

use crate::{
    msg::{QueryMsg, QueryResp},
    state::ADMIN,
};

pub(crate) fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

pub(crate) fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResp> {
    match msg {
        QueryMsg::Donate(_) => Ok(QueryResp::new("foo")),
    }
}
