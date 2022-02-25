//! Contains the Contract struct and its implementation
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, PanicOnDefault};

use payout::Payout;

mod payout;
mod vote;

pub mod views;

/// The main contract governing Ambassadors DAO
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    proposals: LookupMap<u64, Payout>,
    last_proposal_id: u64,
    bounties: LookupMap<u64, Payout>,
    last_bounty_id: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            proposals: LookupMap::<u64, Payout>::new(b"p".to_vec()),
            last_proposal_id: 0,
            bounties: LookupMap::<u64, Payout>::new(b"b".to_vec()),
            last_bounty_id: 0,
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
