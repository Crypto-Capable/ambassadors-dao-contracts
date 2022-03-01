use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};

pub use bounty::{Bounty, BountyPayout};
pub use miscellaneous::{Miscellaneous, MiscellaneousPayout};
pub use proposal::{Proposal, ProposalPayout};

use crate::*;

mod bounty;
mod miscellaneous;
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
    Finalized,
    Removed(Option<String>),
    UnderConsideration,
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

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct VotesCount {
    approve_count: u64,
    reject_count: u64,
}

impl VotesCount {
    pub fn new() -> Self {
        Self {
            approve_count: 0,
            reject_count: 0,
        }
    }
}

/// A Payout is a type of payout. Depeding on the type of the Payout
/// a set of information is required.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout<T> {
    /// the current status of the Payout
    pub status: PayoutStatus,
    /// the id of the account that created the Payout
    pub proposer: AccountId,
    /// the information needed to create a Payout depending on it's kind
    pub info: T,
    /// a brief description for the Payout
    pub description: String,
    /// the of individual votes on the Payout
    pub votes: HashMap<AccountId, vote::Vote>,
    /// the total vote count, updated whenever the votes are updated
    pub votes_count: VotesCount,
}

// TODO: referral payouts
// TODO: implementation of adding proposals, acting on proposals, and executing proposals

#[near_bindgen]
impl Contract {
    #[private]
    pub fn internal_act_payout<T>(
        &self,
        payout: &mut Payout<T>,
        action: types::Action,
        note: Option<String>,
    ) {
        let signer = env::signer_account_id();
        // check if the user is authorized to take the action
        match action {
            types::Action::RemovePayout => {
                if signer != payout.proposer {
                    panic!("{}", error::ErrNotPermitted);
                }
                payout.status = PayoutStatus::Removed(note);
            }
            types::Action::VoteReject => {
                if !self.policy.is_council_member(&signer) {
                    panic!("{}", error::ErrNotPermitted);
                }
                payout.votes.insert(signer, vote::Vote::from(action));
                payout.votes_count.reject_count += 1;
            }
            types::Action::VoteApprove => {
                if !self.policy.is_council_member(&signer) {
                    panic!("{}", error::ErrNotPermitted);
                }
                payout.votes.insert(signer, vote::Vote::from(action));
                payout.votes_count.approve_count += 1;
            }
        };
    }
}
