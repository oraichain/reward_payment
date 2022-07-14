use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidUser user: {user:?}")]
    InvalidUser { user: String},

    #[error("ValidUser user: {user:?}")]
    ValidUser { user: String},

    #[error("Not enough balance current balance: {current_balance:?}, need_balance: {need_balance:?}")]
    NotEnoughBalance { current_balance: u128, need_balance: u128},
}
