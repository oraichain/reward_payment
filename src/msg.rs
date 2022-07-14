use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    // pub token_denom: String,
    pub admin_address: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ActiveUser {addr: HumanAddr, group_name: String},
    DeactiveUser {addr: HumanAddr, group_name: String},
    UpdateReward { group_name: String, new_reward_amount: u32},
    UpdateAdmin {admin_address: HumanAddr},
    Pay {group_name: String, rate: u32}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetInfoGroup {group_name: String},
    GetCurrentBalance {},
    GetInfoAdmin {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InfoResponse {
    pub users: Vec<HumanAddr>,
    pub usd_reward: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BalanceRespone {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AdminRespone {
    pub admin_address: HumanAddr,
}


