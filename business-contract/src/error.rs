use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("only unordered channels are supported")]
    OrderedChannel {},

    #[error("invalid IBC channel version. Got ({actual}), expected ({expected})")]
    InvalidVersion { actual: String, expected: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Ian not Found")]
    IanNotFound {},

    #[error("Invalid Owner Chain")]
    InvalidOwnerChain {},

    #[error("Invalid Settlement Network")]
    InvalidSettlementNetwork {},
}
