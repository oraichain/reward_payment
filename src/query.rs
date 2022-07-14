use std::str::FromStr;

use cosmwasm_std::{Deps, Env, Decimal, Uint128};

use crate::msg::{InfoResponse, BalanceRespone, AdminRespone, MoneyRespone};
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

pub fn get_money_reward(rate: String, usd_reward: u32) -> Result<MoneyRespone, ContractError>{
    let rate_decimal = Decimal::from_str(&rate)? * Uint128::from(1000 as u128);
    let orai_reward = 10_u128.pow(9)* usd_reward as u128 / Uint128::u128(&rate_decimal);

    Ok(MoneyRespone{orai_reward: orai_reward.to_string()})
}
