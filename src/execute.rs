use cosmwasm_std::{attr, DepsMut, MessageInfo, HandleResponse, Env, BankMsg, coin, CosmosMsg, Uint128, StdResult};
use cosmwasm_std::HumanAddr;

use crate::error::ContractError;
use crate::state::{get_admin_address, get_group_info,set_group_info, get_token_info, set_admin_address};

/* 
    active user: add user to group
*/
pub fn active_user(deps: DepsMut, info: MessageInfo, addr: HumanAddr, group_name: String) ->  Result<HandleResponse, ContractError>{
    if get_admin_address(deps.storage).load()? != info.sender {
        return Err(ContractError::Unauthorized {})
    }

    let mut group = get_group_info(deps.storage).load(&group_name.as_bytes())?;

    let mut users = group.users;

    if users.iter().any(|item| item == &addr) {
        return Err(ContractError::ValidUser { user: HumanAddr::to_string(&addr)});
    } else {
        users.push(addr);
        group.users = users;

        set_group_info(deps.storage).save(&group_name.as_bytes(), &group)?;
    }

    Ok(HandleResponse {
        messages: vec![],
        attributes: vec![attr("action", "active_user")],
        data: None,
    })
}

/* 
    deactive user: remove user from group
*/
pub fn deactive_user(deps: DepsMut, info: MessageInfo, addr: HumanAddr, group_name: String) ->  Result<HandleResponse, ContractError>{
    if get_admin_address(deps.storage).load()? != info.sender {
        return Err(ContractError::Unauthorized {})
    }
    
    let mut group = get_group_info(deps.storage).load(&group_name.as_bytes())?;

    let mut users = group.users;

    if users.iter().any(|item| item == &addr) {
        let index_user = users.iter().position(|item| *item == addr).unwrap();
        users.remove(index_user);
        group.users = users;
        set_group_info(deps.storage).save(&group_name.as_bytes(), &group)?;

    } else {
        return Err(ContractError::InvalidUser { user: HumanAddr::to_string(&addr) });
    }
    Ok(HandleResponse {
        messages: vec![],
        attributes: vec![attr("action", "deactive_user")],
        data: None,
    })
}

/* 
    update reward: update reward by group name
*/

pub fn update_admin(deps: DepsMut, info: MessageInfo, admin_address: HumanAddr) -> Result<HandleResponse, ContractError>{
    if get_admin_address(deps.storage).load()? != info.sender {
        return Err(ContractError::Unauthorized {})
    }

    set_admin_address(deps.storage).save(&admin_address)?;

    Ok(HandleResponse {
        messages: vec![],
        attributes: vec![attr("action", "update admin")],
        data: None,
    })

}
pub fn update_reward(deps: DepsMut, info: MessageInfo, group_name: String, new_reward_amount: u32) ->  Result<HandleResponse, ContractError>{
    if get_admin_address(deps.storage).load()? != info.sender {
        return Err(ContractError::Unauthorized {})
    }
    

    let mut group = get_group_info(deps.storage).load(&group_name.as_bytes())?;
    group.usd_reward = new_reward_amount;

    set_group_info(deps.storage).save(&group_name.as_bytes(), &group)?;

    Ok(HandleResponse {
        messages: vec![],
        attributes: vec![attr("action", "update reward")],
        data: None,
    })

}

/* 
    pay reward: send token to wallets in group by rate, refund token to sender
*/
pub fn pay_reward(deps: DepsMut, env: Env, info: MessageInfo,group_name: String, rate: u32) ->  Result<HandleResponse, ContractError>{
    if get_admin_address(deps.storage).load()? != info.sender {
        return Err(ContractError::Unauthorized {})
    }

    let group = get_group_info(deps.storage).load(&group_name.as_bytes())?;
    let reward_amount = group.usd_reward;

    let users = group.users;

    let sent_balance = Uint128::u128(&info.sent_funds[0].amount);

    let _token_denom = get_token_info(deps.storage).load()?;

    let need_balance = ((users.len() as u32) * rate * reward_amount) as u128;


    if sent_balance < need_balance {
        return Err(ContractError::NotEnoughBalance{current_balance: sent_balance, need_balance: need_balance});
    }
    
    let mut transfer_messages:Vec<CosmosMsg> = vec![];
    let sender = HumanAddr::to_string(&env.contract.address);

    for wallet_addr in users.iter() {
        let amout_to_fund = rate * reward_amount ;
        let sender_addr = HumanAddr::from(sender.clone());
        let reciever_addr = HumanAddr::from(wallet_addr);

        let msg_succes = send_token(amout_to_fund as u128, sender_addr, reciever_addr, &_token_denom.clone())?;
        transfer_messages.push(msg_succes);
    
    }

    let refund_balance = sent_balance - need_balance;
    let msg_success = send_token(refund_balance, HumanAddr::from(sender.clone()), info.sender, &_token_denom.clone())?;

    transfer_messages.push(msg_success);


    Ok(HandleResponse {
        messages: transfer_messages,
        attributes: vec![attr("action", "pay_reward")],
        data: None,
    })
}

// Private Function
fn send_token(amount: u128, from_address: HumanAddr, to_address: HumanAddr, denom: &str) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Bank(BankMsg::Send {
        from_address: from_address,
        to_address: to_address,
        amount: vec![coin(amount, denom)],
    }))
}

