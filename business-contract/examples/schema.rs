use cosmwasm_schema::write_api;
//
// use business_contract::msg::{ResolvedIanResponse,IbcQueryMsg,HasKycedResponse,ExecuteMsg, InstantiateMsg, QueryMsg};
//
// fn main() {
//     write_api! {
//         instantiate: InstantiateMsg,
//         query: QueryMsg,
//         execute: ExecuteMsg,
//         resolved_ian: ResolvedIanResponse,
//         ibc_query: IbcQueryMsg,
//         has_kyced: HasKycedResponse
//     }
// }

use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use business_contract::msg::{ResolvedIanResponse,IbcQueryMsg,HasKycedResponse,ExecuteMsg, InstantiateMsg, QueryMsg};
use business_contract::state::Ian;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Ian), &out_dir);
    export_schema(&schema_for!(ResolvedIanResponse), &out_dir);
    export_schema(&schema_for!(IbcQueryMsg), &out_dir);
    export_schema(&schema_for!(HasKycedResponse), &out_dir);
}
