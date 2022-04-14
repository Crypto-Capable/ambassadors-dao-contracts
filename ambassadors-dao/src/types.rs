use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{Balance, ONE_NEAR};

pub type ReferralToken = String;

pub type USD = f64;

pub(crate) fn usd_to_balance(amount: USD, rate: f64) -> Balance {
    let mut total_usd = amount * rate;
    let mut divisor = 1_u128;
    while total_usd.fract() > 1e-8 {
        total_usd *= 10_f64;
        divisor *= 10;
    }
    (total_usd.trunc() as u128) * (ONE_NEAR / divisor)
}

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

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RegistrationResult {
    pub status: bool,
    pub message: Option<String>,
}

impl RegistrationResult {
    pub fn new(status: bool, message: Option<String>) -> Self {
        Self { status, message }
    }
}
