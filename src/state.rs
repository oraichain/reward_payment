use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Storage, HumanAddr};
// use cw_storage_plus::{Map, Item};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket,
    ReadonlyBucket, ReadonlySingleton, Singleton,
};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Group {
    pub users: Vec<HumanAddr>,
    pub usd_reward: u32,
}


// pub const REWARD_PAYEMENT_INFO: Map<&str, Group> = Map::new("REWARD_PAYEMENT_INFO");
// pub const TOKEN_INFO: Item<HumanAddr> = Item::new("TOKEN_INFO");
// pub const ADMIN_ADDR: Item<HumanAddr> = Item::new("ADMIN_ADDRESS");


pub fn set_admin_address(storage: &mut dyn Storage) -> Singleton<HumanAddr> {
    singleton(storage, b"ADMIN_ADDRESS")
}

pub fn get_admin_address(storage: &dyn Storage)-> ReadonlySingleton<HumanAddr> {
    singleton_read(storage, b"ADMIN_ADDRESS")
}

pub fn get_token_info(storage: &dyn Storage) -> ReadonlySingleton<String> {
    singleton_read(storage, b"TOKEN_INFO")
}

pub fn set_token_info(storage: &mut dyn Storage) -> Singleton<String> {
    singleton(storage, b"TOKEN_INFO")
}

pub fn get_group_info(storage: &dyn Storage) -> ReadonlyBucket<Group> {
    bucket_read(storage, b"REWARD_PAYEMENT_INFO")
}


pub fn set_group_info(storage: &mut dyn Storage) -> Bucket<Group> {
    bucket(storage, b"REWARD_PAYEMENT_INFO")
}