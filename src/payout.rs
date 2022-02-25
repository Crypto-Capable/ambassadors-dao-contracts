use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use crate::vote::Vote;

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
    UnderConsideration,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind {
    Hackathon {
        expected_registrations: u64,
        estimated_budget: u64,
        supporting_document: ResourceLink,
    },
    MemeContest {
        expected_registrations: u64,
        estimated_budget: u64,
        supporting_document: ResourceLink,
    },
    Open {
        supporting_document: ResourceLink,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum BountyKind {
    HackathonCompletion {
        num_of_registrations: u64,
        num_of_submissions: u64,
        // order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    MemeContestCompletion {
        num_of_submissions: u64,
        // order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    Webinar {
        num_of_registrations: u64,
        num_of_attendees: u64,
        webinar_link: ResourceLink,
    },
    ContentCoordniation {
        content_links: Vec<ResourceLink>,
        story: String,
        tools_used: Vec<String>,
    },
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

// TODO: referrals and miscellaneous payout info kinds

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
    pub votes: HashMap<AccountId, Vote>,
    /// the total vote count, updated whenever the votes are updated
    pub votes_count: u64,
}

pub fn make() {
    let p = Payout {
        info: PayoutInfo::Proposal(ProposalKind::Hackathon {
            expected_registrations: 128,
            estimated_budget: 2000,
            supporting_document: "asd".to_string(),
        }),
        description: "asd".to_string(),
        proposer: AccountId::new_unchecked("asd".to_string()),
        status: PayoutStatus::UnderConsideration,
        votes: HashMap::new(),
        votes_count: 0,
    };
}
