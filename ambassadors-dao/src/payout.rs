use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use crate::vote::Vote;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum PayoutStatus {
    Approved,
    Rejected,
    Removed,
    Expired,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum PayoutKind {
    Proposal,
    Bounty,
    Referral,
    Miscellaneous,
}

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

/// Different kinds of Information required for different kinds of payouts
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum PayoutInfo {
    // proposals
    HackathonProposal {
        expected_registrations: U64,
        estimated_budget: U64,
        supporting_document: ResourceLink,
    },
    MemeContestProposal {
        expected_registrations: U64,
        estimated_budget: U64,
        supporting_document: ResourceLink,
    },
    OpenProposal {
        supporting_document: ResourceLink,
    },
    // bounties
    HackathonCompletionBounty {
        num_of_registrations: U64,
        num_of_submissions: U64,
        // order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    MemeContestCompletionBounty {
        num_of_submissions: U64,
        // order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    WebinarBounty {
        num_of_registrations: U64,
        num_of_attendees: U64,
        webinar_link: ResourceLink,
    },
    ContentCoordniationBounty {
        content_links: Vec<ResourceLink>,
        story: String,
        tools_used: Vec<String>,
    },
    // TODO: enable these
    // referrals
    // CampusAmbasaddorReferral {},
    // NearCertifiedDeveloperReferral {},
    // RecruitementReferral {},
    // miscellaneous
    // ContentCreationBounty {},
    // CampusMOUSigning {},
    // CampusAmbassadorBonus {},
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
    /// the kind of the Payout
    pub kind: PayoutKind,
    /// the information needed to create a Payout
    pub info: PayoutInfo,
    /// the brief description for the Payout
    pub description: String,
    /// the of individual votes on the Payout
    pub votes: HashMap<AccountId, Vote>,
    /// the total vote count, updated whenever the votes are updated
    pub votes_count: U64,
}
