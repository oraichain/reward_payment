use cosmwasm_std::{to_binary, Binary, DepsMut, Env, Deps,
    HandleResponse, InitResponse, MessageInfo, StdResult, HumanAddr
};

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{set_token_info, set_group_info, set_admin_address, Group};
use crate::execute::{active_user, deactive_user, update_reward, pay_reward, update_admin};
use crate::query::{get_info_group, get_current_balance, get_info_admin, get_money_reward};
use crate::config::{AI_EXECUTOR_WALLETS, VRF_WALLETS, AI_EXECUTOR_USD_REWARD, VRF_USD_REWARD, TOKEN_DENOM, AI_EXECUTOR_SERVICE_NAME, VRF_SERVICE_NAME};


pub fn init(deps: DepsMut, _env: Env, info: MessageInfo, _msg: InitMsg) -> StdResult<InitResponse> {
    let ai_executor_addrs: Vec<HumanAddr> = AI_EXECUTOR_WALLETS.iter().map(|wallet_addr| HumanAddr::from(wallet_addr.to_string())).collect();
    let vrf_addrs: Vec<HumanAddr> = VRF_WALLETS.iter().map(|wallet_addr| HumanAddr::from(wallet_addr.to_string())).collect();

    let ai_executor_group = Group{users: ai_executor_addrs, usd_reward: AI_EXECUTOR_USD_REWARD};
    let vrf_group = Group{users: vrf_addrs, usd_reward: VRF_USD_REWARD};

    set_group_info(deps.storage).save(AI_EXECUTOR_SERVICE_NAME.as_bytes(), &ai_executor_group)?;
    set_group_info(deps.storage).save(VRF_SERVICE_NAME.as_bytes(), &vrf_group)?;


    set_admin_address(deps.storage).save(&info.sender)?;
    set_token_info(deps.storage).save(&TOKEN_DENOM.to_string())?;

    Ok(InitResponse::default())
}

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        HandleMsg::ActiveUser {addr, group_name} => active_user(deps, info, addr, group_name),
        HandleMsg::DeactiveUser { addr, group_name } => deactive_user(deps, info, addr, group_name),
        HandleMsg::UpdateReward { group_name, new_reward_amount } => update_reward(deps, info, group_name, new_reward_amount),
        HandleMsg::UpdateAdmin { admin_address } => update_admin(deps, info, admin_address),
        HandleMsg::Pay { group_name, rate, special_group } => pay_reward(deps, env, info, group_name, rate, special_group)

    }
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let response = match msg {
        QueryMsg::GetInfoGroup {group_name} => to_binary(&get_info_group(deps, group_name)?)?,
        QueryMsg::GetCurrentBalance {} => to_binary(&get_current_balance(deps, env)?)?,
        QueryMsg::GetInfoAdmin {} => to_binary(&get_info_admin(deps)?)?,
        QueryMsg::GetMoneySwap { rate, usd_reward } => to_binary(&get_money_reward(rate, usd_reward)?)?,
        
    };
    Ok(response)
}


// orai14n3tx8s5ftzhlxvq0w5962v60vd82h30rha573