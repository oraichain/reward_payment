use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use reward_payment::msg::{HandleMsg, InitMsg, QueryMsg, InfoResponse, BalanceRespone, AdminRespone, MoneyRespone, MigrateMsg};
use reward_payment::state::{Group, SpecialGroup};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Group), &out_dir);
    // export_schema(&schema_for!(SpecialGroup), &out_dir);
    export_schema(&schema_for!(InfoResponse), &out_dir);
    export_schema(&schema_for!(BalanceRespone), &out_dir);
    export_schema(&schema_for!(AdminRespone), &out_dir);
    export_schema(&schema_for!(MoneyRespone), &out_dir);
}
