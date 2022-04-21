use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{Balance, ONE_NEAR};

pub const ONE_TGAS: u64 = 1_000_000_000_000;

pub type ReferralToken = String;

#[allow(clippy::upper_case_acronyms)]
pub type USD = f64;

pub(crate) fn usd_to_balance(amount: f64, rate: f64) -> Balance {
    let mut near_tokens = amount / rate;
    let mut divisor = 1_u128;
    while near_tokens.fract() > 0.0001 {
        near_tokens *= 10_f64;
        divisor *= 10;
    }
    (near_tokens.trunc() as u128) * (ONE_NEAR / divisor)
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

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum RegistrationResult {
    SuccessWithReferral(u64),
    SuccessWithoutReferral(u64, String),
}
