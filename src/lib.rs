pub mod contract;
pub mod error;
pub mod msg;
pub mod state;
pub mod config;
pub mod execute;
pub mod query;

#[cfg(all(target_arch = "wasm32", not(feature = "library")))]
cosmwasm_std::create_entry_points_with_migration!(contract);
