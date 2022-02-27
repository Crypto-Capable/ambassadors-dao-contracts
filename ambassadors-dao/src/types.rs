use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{Balance, Gas};

/// 1 yN to prevent access key fraud.
pub const ONE_YOCTO_NEAR: Balance = 1;

/// Gas for single ft_transfer call.
pub const GAS_FOR_FT_TRANSFER: Gas = Gas(10_000_000_000_000);

/// Configuration of the DAO.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    /// name of DAO
    pub name: String,
    /// purpose of DAO
    pub purpose: String,
}

impl Config {
    pub fn new(name: String, purpose: String) -> Self {
        Self { name, purpose }
    }
}

/// The actions that the members of the DAO can perform such as
/// adding a new prosposal or voting for a proposal, etc...
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Action {
    /// adding a payout
    AddPayout,
    /// remove a payout
    RemovePayout,
    /// approval vote for a payout
    VoteApprove,
    /// rejection vote for a payout
    VoteReject,
}

impl Action {
    pub fn to_policy_label(&self) -> String {
        format!("{:?}", self)
    }
}
