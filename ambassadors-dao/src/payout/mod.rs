use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use bounty::BountyKind;
use proposal::ProposalKind;

use crate::*;

mod bounty;
mod proposal;

/// The URL to any resource on the internet
pub type ResourceLink = String;

/// Represents a submission
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct SubmissionInfo {
    name: String,
    account_id: AccountId,
    submission_link: ResourceLink,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum PayoutStatus {
    Approved,
    Rejected,
    Removed,
    Expired,
    InProgress,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum PayoutInfo {
    Proposal(ProposalKind),
    Bounty(BountyKind),
    Referral,
    Miscellaneous,
}

/// A generic input structure for payouts
///
/// Let's say you want to add a proposal payout
/// ```rust
/// pub fn add_proposal(&mut self, proposal: PayoutInput<ProposalKind>) {
///     // do something here
/// }
/// ```
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct PayoutInput<T> {
    description: String,
    information: T,
}

/// A Payout is a type of payout. Depeding on the type of the Payout
/// a set of information is required.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    /// the current status of the Payout
    pub status: PayoutStatus,
    /// the id of the account that created the Payout
    pub proposer: AccountId,
    /// the information needed to create a Payout depending on it's kind
    pub info: PayoutInfo,
    /// a brief description for the Payout
    pub description: String,
    /// the of individual votes on the Payout
    pub votes: HashMap<AccountId, vote::Vote>,
    /// the total vote count, updated whenever the votes are updated
    pub votes_count: u64,
}

// TODO: referrals and miscellaneous payout info kinds
// TODO: implementation of adding proposals, acting on proposals, and executing proposals
// TODO: figure out where the tokens are stored in a smart contract
// TODO: figure out roles and permissions

#[near_bindgen]
impl Contract {}
