use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

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
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct PayoutInput<T: Serialize> {
    pub description: String,
    pub information: T,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct VotesCount {
    pub approve_count: u64,
    pub reject_count: u64,
}

impl VotesCount {
    pub fn new() -> Self {
        Self {
            approve_count: 0,
            reject_count: 0,
        }
    }
}

impl Default for VotesCount {
    fn default() -> Self {
        Self::new()
    }
}

/// A Payout is a type of payout. Depeding on the type of the Payout
/// a set of information is required.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout<T: Serialize> {
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

pub(crate) fn internal_act_payout<T: Serialize>(
    is_council_member: bool,
    council_size: u64,
    payout: &mut Payout<T>,
    action: types::Action,
    note: Option<String>,
) {
    let signer = env::signer_account_id();

    match action {
        types::Action::RemovePayout => {
            // only the proposer can remove the payout
            if signer != payout.proposer {
                panic!("{}", error::ERR_NOT_PERMITTED);
            }
            // payout can only be removed if it is currently under consideration
            match payout.status {
                PayoutStatus::UnderConsideration => {
                    payout.status = PayoutStatus::Removed(note);
                }
                _ => {
                    panic!(
                        "{}: {}",
                        error::ERR_NOT_PERMITTED,
                        "payout not under consideration"
                    );
                }
            };
        }
        types::Action::VoteReject => {
            // check if the user is authorized to take the action
            if !is_council_member {
                panic!("{}", error::ERR_NOT_PERMITTED);
            }
            // if the signer has already voted
            if payout.votes.contains_key(&signer) {
                panic!("{}: {}", error::ERR_NOT_PERMITTED, "already voted");
            }
            // one may think we need to check if the count is consistent with
            // the number of council members, but just checking if the signer
            // council member has voted or not rules out the said issue
            payout.votes.insert(signer, vote::Vote::from(action));
            payout.votes_count.reject_count += 1;
            // update payout status if needed
            internal_update_payout_status(council_size, payout);
        }
        types::Action::VoteApprove => {
            // check if the user is authorized to take the action
            if !is_council_member {
                panic!("{}", error::ERR_NOT_PERMITTED);
            }
            // if the signer has already voted
            if payout.votes.contains_key(&signer) {
                panic!("{}: {}", error::ERR_NOT_PERMITTED, "already voted");
            }
            // one may think we need to check if the count is consistent with
            // the number of council members, but just checking if the signer
            // council member has voted or not rules out the said issue
            payout.votes.insert(signer, vote::Vote::from(action));
            payout.votes_count.approve_count += 1;
            // update payout status if needed
            internal_update_payout_status(council_size, payout);
        }
    };
}

/// check the votes on a payout and update the status if needed
pub(crate) fn internal_update_payout_status<T: Serialize>(
    council_size: u64,
    payout: &mut Payout<T>,
) {
    let approve_count = payout.votes_count.approve_count;
    let reject_count = payout.votes_count.reject_count;
    let sum = approve_count + reject_count;

    // let's check if all council members have voted
    if sum == council_size {
        // if approve_count / sum >= 1/2     (i.e. at least half votes are approve)
        // therefore, 2 * approve_count >= sum
        if 2 * approve_count >= sum {
            payout.status = PayoutStatus::Approved;
        } else {
            payout.status = PayoutStatus::Rejected;
        }
    }
}
