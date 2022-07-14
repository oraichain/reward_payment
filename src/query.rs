use cosmwasm_std::{Deps, Env};

use crate::msg::{InfoResponse, BalanceRespone, AdminRespone};
use crate::state::{get_group_info, get_admin_address};
use crate::error::ContractError;

pub fn get_info_group(deps: Deps, group_name: String) -> Result<InfoResponse, ContractError>{
    let group = get_group_info(deps.storage).load(&group_name.as_bytes())?;
    Ok(InfoResponse{users: group.users, usd_reward: group.usd_reward})
}

pub fn get_info_admin(deps: Deps) -> Result<AdminRespone, ContractError>{
    let admin_address = get_admin_address(deps.storage).load()?;
    Ok(AdminRespone {admin_address: admin_address})
}

pub fn get_current_balance(deps: Deps, env: Env) -> Result<BalanceRespone, ContractError>{
    let balance = deps.querier.query_balance(env.contract.address, "orai")?;
    Ok(BalanceRespone{balance: balance.amount})
}