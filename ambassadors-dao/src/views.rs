use std::cmp::min;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::CryptoHash;

use payout::{Bounty, Proposal};

use crate::*;

/// TODO: Get payout input fields, for that create a derive proc_macro
/// and put it on each of the enums and required structs

/// This is format of output via JSON for the payout.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PayoutOutput<T> {
    /// Id of the payout.
    pub id: u64,
    #[serde(flatten)]
    pub payout: Payout<T>,
}

#[near_bindgen]
impl Contract {
    /// Returns semver of this contract.
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Returns config of this contract.
    pub fn get_config(&self) -> Config {
        self.config.clone()
    }

    /// Returns policy of this contract.
    pub fn get_policy(&self) -> Policy {
        self.policy.clone()
    }

    /// Returns if blob with given hash is stored.
    pub fn has_blob(&self, hash: Base58CryptoHash) -> bool {
        env::storage_has_key(&CryptoHash::from(hash))
    }

    /// Returns locked amount of NEAR that is used for storage.
    pub fn get_locked_storage_amount(&self) -> U128 {
        let locked_storage_amount = env::storage_byte_cost() * (env::storage_usage() as u128);
        U128(locked_storage_amount)
    }

    /// Returns available amount of NEAR that can be spent (outside of amount for storage and bonds).
    // pub fn get_available_amount(&self) -> U128 {
    //     U128(env::account_balance() - self.get_locked_storage_amount().0 - self.locked_amount)
    // }

    /// Get the number of proposals, also happens to be the ID of the latest proposal
    pub fn get_last_proposal_id(&self) -> u64 {
        self.last_proposal_id
    }

    /// Get proposals in paginated view.
    pub fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<PayoutOutput<Proposal>> {
        (from_index..min(self.last_proposal_id, from_index + limit))
            .filter_map(|id| {
                self.proposals
                    .get(&id)
                    .map(|p| PayoutOutput { id, payout: p })
            })
            .collect()
    }

    /// Get specific proposal.
    pub fn get_proposal(&self, id: u64) -> PayoutOutput<Proposal> {
        let proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
        PayoutOutput {
            id,
            payout: proposal,
        }
    }

    /// Get specific bounty
    pub fn get_bounty(&self, id: u64) -> PayoutOutput<Bounty> {
        let bounty = self.bounties.get(&id).expect("ERR_NO_BOUNTY");
        PayoutOutput { id, payout: bounty }
    }

    /// Get the number of bounties, also happens to be the ID of the latest bounty
    pub fn get_last_bounty_id(&self) -> u64 {
        self.last_bounty_id
    }

    /// Get bounties in paginated view.
    pub fn get_bounties(&self, from_index: u64, limit: u64) -> Vec<PayoutOutput<Bounty>> {
        (from_index..std::cmp::min(from_index + limit, self.last_bounty_id))
            .filter_map(|id| {
                self.bounties
                    .get(&id)
                    .map(|p| PayoutOutput { id, payout: p })
            })
            .collect()
    }
}
