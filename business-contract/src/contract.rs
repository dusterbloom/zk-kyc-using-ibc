
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response, StdResult, StdError,
};
use cw2::set_contract_version;
use cosmwasm_schema::{cw_serde, QueryResponses};
use pseudo_IAN::{IAN, EntityType};

use crate::{error::ContractError, state::{IANS_SEQ, WHITELIST_MAP, Ian, IANS}, msg::{InstantiateMsg, ExecuteMsg, IbcQueryMsg, QueryMsg, HasKycedResponse, ResolvedIanResponse}, pseudo_IAN};

const CONTRACT_NAME: &str = "crates.io:business-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    IANS_SEQ.save(deps.storage, &0)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute_ian_create(
    deps: DepsMut,
    owner_chain: String,
    owner_address: String,
    application_chain: String,
    application_address: String,
) -> Result<Response, ContractError> {

    // Define the entity type based on your specific requirements
    let entity_type = EntityType::HumanOrganization; // Example

    // Create the IAN
    let ian = IAN::new(
        owner_chain.as_str(),
        owner_address.as_str(),
        application_address.as_str(),
        application_chain.as_str(),
        entity_type,
        "VAT12345657" // Example entity ID
    );

    // Convert the IAN to a string
    let raw_ian = ian.to_string();

    // Increment the counter and save the new Encrypted RecordId
    let id = IANS_SEQ.load(deps.storage)? + 1;
    IANS_SEQ.save(deps.storage, &id)?;

    // Create the record
    let record = Ian {
        owner_address,
        owner_chain,
        application_address,
        application_chain,
        ian: raw_ian,
        id
    };

   

    IANS.save(deps.storage, record.ian.to_string(), &record)?;

    Ok(Response::new().add_attribute("action", "ian_created"))
}




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HasKyced { address } => query_get_valid_address(deps, address),
        QueryMsg::ResolveIan { ian } => query_resolver(deps,_env, ian),
    }
}

pub fn query_get_valid_address(deps: Deps, address: String) -> StdResult<Binary> {
    let is_address_present = WHITELIST_MAP
        .may_load(deps.storage, address)?
        .unwrap_or_else(|| false);

    if is_address_present {
        to_binary(&HasKycedResponse {
            result: "address is KYCed".to_string(),
        })
    } else {
        to_binary(&HasKycedResponse {
            result: "address is NOT KYCed".to_string(),
        })
    }
}


fn query_resolver(deps: Deps, _env: Env, ian: String) -> StdResult<Binary> {
    let ian_record = IANS.may_load(deps.storage, ian)?
        .ok_or_else(|| StdError::not_found("IAN"))?; // Return an error if not found

    let resp = ResolvedIanResponse { result: ian_record };

    to_binary(&resp)
}


#[cfg(test)]
mod tests {}
