use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Kyc {
        channel: String,
        proof: String,
        address: String,
        public_signal: u64,
    },
    IbcAcknowledgeKyc {
        is_valid: bool,
        address: String,
    },
    Ian {
        owner_chain: String,
        owner_address: String,
        application_chain: String,
        application_address: String,
        // metadata: String,
        // settlement_network: String,
        // settlement_address: String,
    },
}

#[cw_serde]
pub enum IbcQueryMsg {
    Verify {
        proof: String,
        address: String,
        public_signal: u64,
    },
}

#[cw_serde]
pub enum IbcBizExecuteMsg {
    DoKyc { address: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(HasKycedResponse)]
    HasKyced { address: String },
    #[returns(HasKycedResponse)]
    ResolveIan { ian: String },
}

#[cw_serde]
pub struct HasKycedResponse {
    pub result: String,
}

#[cw_serde]
pub struct ResolveIanResponse {
    pub result: String,
}
