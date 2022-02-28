//! Contains the Contract struct and its implementation
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, PanicOnDefault};

use payout::{BountyPayout, MiscellaneousPayout, Payout, ProposalPayout};
use policy::Policy;
use types::Config;

mod payout;
mod policy;
mod types;
mod vote;

pub mod views;

// TODO: create a proc_macro for generate meta data about the type of information
// that each Payout type needs for creation

/// The main contract governing Ambassadors DAO
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    policy: Policy,
    config: Config,
    proposals: LookupMap<u64, ProposalPayout>,
    last_proposal_id: u64,
    bounties: LookupMap<u64, BountyPayout>,
    last_bounty_id: u64,
    miscellaneous: LookupMap<u64, MiscellaneousPayout>,
    last_miscellaneous_id: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct CreateContractParams {
    council: Vec<AccountId>,
    name: String,
    purpose: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(params: CreateContractParams) -> Self {
        Self {
            policy: Policy::from(params.council),
            config: Config::new(params.name, params.purpose),
            proposals: LookupMap::<u64, ProposalPayout>::new(b"p".to_vec()),
            last_proposal_id: 0,
            bounties: LookupMap::<u64, BountyPayout>::new(b"b".to_vec()),
            last_bounty_id: 0,
            miscellaneous: LookupMap::<u64, MiscellaneousPayout>::new(b"m".to_vec()),
            last_miscellaneous_id: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_contract() {
        unimplemented!()
    }
}
