//! Contains the Contract struct and it's implementation
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, PanicOnDefault};

use payout::Payout;

mod payout;
pub mod views;
mod vote;

/// The main contract governing Ambassadors DAO
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    proposals: LookupMap<u64, Payout>,
    bounties: LookupMap<u64, Payout>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            bounties: LookupMap::<u64, Payout>::new(b"b".to_vec()),
            proposals: LookupMap::<u64, Payout>::new(b"p".to_vec()),
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
