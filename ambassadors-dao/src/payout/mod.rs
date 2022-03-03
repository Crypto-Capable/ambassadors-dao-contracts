use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};

pub use bounty::{Bounty, BountyPayout};
pub use miscellaneous::{Miscellaneous, MiscellaneousPayout};
pub use proposal::{Proposal, ProposalPayout};
pub use referrals::{Referral, ReferralPayout};

use crate::*;

mod bounty;
mod miscellaneous;
mod proposal;
mod referrals;

/// The URL to any resource on the internet
pub type ResourceLink = String;

/// Represents a submission
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct SubmissionInfo {
    pub name: String,
    pub account_id: AccountId,
    pub submission_link: ResourceLink,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum PayoutStatus {
    Approved,
    Rejected,
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
    /// Acts on the payout according to the action passed.
    /// Also changes the status of the payout if deemed so.
    #[private]
    pub fn internal_act_payout<T>(
        &self,
        payout: &mut Payout<T>,
        action: types::Action,
        note: Option<String>,
    ) {
        let signer = env::signer_account_id();

        match action {
            types::Action::RemovePayout => {
                // only the proposer can remove the payout
                if signer != payout.proposer {
                    panic!("{}", error::ErrNotPermitted);
                }
                // payout can only be removed if it is currently under consideration
                match payout.status {
                    PayoutStatus::UnderConsideration => {
                        payout.status = PayoutStatus::Removed(note);
                    }
                    _ => {
                        panic!(
                            "{}: {}",
                            error::ErrNotPermitted,
                            "payout not under consideration"
                        );
                    }
                };
            }
            types::Action::VoteReject => {
                // check if the user is authorized to take the action
                if !self.policy.is_council_member(&signer) {
                    panic!("{}", error::ErrNotPermitted);
                }
                // if the signer has already voted
                if payout.votes.contains_key(&signer) {
                    panic!("{}: {}", error::ErrNotPermitted, "already voted");
                }
                // one may think we need to check if the count is consistent with
                // the number of council members, but just checking if the signer
                // council member has voted or not rules out the said issue
                payout.votes.insert(signer, vote::Vote::from(action));
                payout.votes_count.reject_count += 1;
                // update payout status if needed
                self.internal_update_payout_status(payout);
            }
            types::Action::VoteApprove => {
                // check if the user is authorized to take the action
                if !self.policy.is_council_member(&signer) {
                    panic!("{}", error::ErrNotPermitted);
                }
                // if the signer has already voted
                if payout.votes.contains_key(&signer) {
                    panic!("{}: {}", error::ErrNotPermitted, "already voted");
                }
                // one may think we need to check if the count is consistent with
                // the number of council members, but just checking if the signer
                // council member has voted or not rules out the said issue
                payout.votes.insert(signer, vote::Vote::from(action));
                payout.votes_count.approve_count += 1;
                // update payout status if needed
                self.internal_update_payout_status(payout);
            }
        };
    }

    /// check the votes on a payout and update the status if needed
    #[private]
    pub fn internal_update_payout_status<T>(&self, payout: &mut Payout<T>) {
        let approve_count = payout.votes_count.approve_count;
        let reject_count = payout.votes_count.reject_count;
        let sum = approve_count + reject_count;

        // let's check if all council members have voted
        if sum == self.policy.get_council_size() as u64 {
            // if approve_count / sum >= 1/2     (i.e. at least half votes are approve)
            // therefore, 2 * approve_count >= sum
            if 2 * approve_count >= sum {
                payout.status = PayoutStatus::Approved;
            } else {
                payout.status = PayoutStatus::Rejected;
            }
        }
    }
}
