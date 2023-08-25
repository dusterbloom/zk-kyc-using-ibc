use cosmwasm_std::entry_point;

use crate::{error::ContractError, msg::ExecuteMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use pseudo_ian::IAN;

use crate::{
    msg::{HasKycedResponse, InstantiateMsg, QueryMsg, ResolvedIanResponse},
    pseudo_ian,
    state::{Ian, IANS, IANS_SEQ, WHITELIST_MAP},
};

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

pub fn execute(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ian {
            owner_chain,
            owner_address,
            application_chain,
            application_address,
            settlement_network,
            settlement_address,
            private,
        } => execute_ian_create(
            deps,
            owner_chain,
            owner_address,
            application_chain,
            application_address,
            settlement_network,
            settlement_address,
            private,
        ),
        ExecuteMsg::IbcAcknowledgeKyc { is_valid, address } => todo!(),

        ExecuteMsg::Kyc {
            channel,
            proof,
            address,
            public_signal,
        } => todo!(),
    }
}

// Define valid owner chains (cryptocurrencies) and settlement networks
const VALID_OWNER_CHAINS: &[&str] = &[
    "BTC", "ETH", "COSMOS", "COFI", "HID", "CHEQD", "NAMADA", "AKASH",
];
const VALID_SETTLEMENT_NETWORKS: &[&str] = &["IBAN", "Visa", "Mastercard", "SIXD", "CASH"];

pub fn execute_ian_create(
    deps: DepsMut,
    owner_chain: String,
    owner_address: String,
    application_chain: String,
    application_address: String,
    settlement_chain: String,
    settlement_address: String,
    private: bool,
) -> Result<Response, ContractError> {
    // Validate owner chain
    if !VALID_OWNER_CHAINS.contains(&owner_chain.as_str()) {
        return Err(ContractError::InvalidOwnerChain {});
    }

    // Validate settlement network (can be either a valid owner chain or a specific settlement network)
    if !VALID_OWNER_CHAINS.contains(&settlement_chain.as_str())
        && !VALID_SETTLEMENT_NETWORKS.contains(&settlement_chain.as_str())
    {
        return Err(ContractError::InvalidSettlementNetwork {});
    }

    // Create the IAN
    let ian = IAN::new(
        owner_chain.as_str(),
        owner_address.as_str(),
        application_address.as_str(),
        application_chain.as_str(),
        settlement_chain.as_str(),
        settlement_address.as_str(),
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
        settlement_chain,
        settlement_address,
        ian: raw_ian,
        id,
        private,
    };

    IANS.save(deps.storage, record.ian.to_string(), &record)?;

    Ok(Response::new()
        .add_attribute("action", "ian_created")
        .add_attribute("ian", ian.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HasKyced { address } => query_get_valid_address(deps, address),
        QueryMsg::ResolveIan { ian } => query_resolver(deps, ian),
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

pub fn query_resolver(deps: Deps, ian: String) -> StdResult<Binary> {
    let ian_record = IANS
        .may_load(deps.storage, ian)?
        .ok_or_else(|| StdError::not_found("IAN not found"))?; // Return an error if not found

    // Check if the IAN is private
    if ian_record.private {
        // Return a limited response
        return to_binary(&ResolvedIanResponse {
            result: Err("IAN exists but it is private".to_string()),
        });
    }

    let resp = ResolvedIanResponse {
        result: Ok(ian_record),
    };

    to_binary(&resp)
}

#[cfg(test)]
mod tests {}
