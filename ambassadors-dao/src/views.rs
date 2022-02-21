use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use std::cmp::min;

use crate::*;

/// This is format of output via JSON for the proposal.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalOutput {
    /// Id of the proposal.
    pub id: u64,
    #[serde(flatten)]
    pub proposal: Proposal,
}

/// This is format of output via JSON for the bounty.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BountyOutput {
    /// Id of the bounty.
    pub id: u64,
    #[serde(flatten)]
    pub bounty: Bounty,
}

#[near_bindgen]
impl Contract{
    pub fn get_config(&self) -> Config {
        self.config.get().unwrap().clone()
    }

    /// Returns policy of this contract.
    pub fn get_policy(&self) -> Policy {
        self.policy.get().unwrap().to_policy().clone()
    }
    pub fn get_available_amount(&self) -> U128 {
        U128(env::account_balance() - self.get_locked_storage_amount().0 - self.locked_amount)
    }
    pub fn get_last_proposal_id(&self) -> u64 {
        self.last_proposal_id
    }

    /// Get proposals in paginated view.
    pub fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<ProposalOutput> {
        (from_index..min(self.last_proposal_id, from_index + limit))
            .filter_map(|id| {
                self.proposals.get(&id).map(|proposal| ProposalOutput {
                    id,
                    proposal: proposal.into(),
                })
            })
            .collect()
    }

    /// Get specific proposal.
    pub fn get_proposal(&self, id: u64) -> ProposalOutput {
        let proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
        ProposalOutput {
            id,
            proposal: proposal.into(),
        }
    }

    /// Get given bounty by id.
    pub fn get_bounty(&self, id: u64) -> BountyOutput {
        let bounty = self.bounties.get(&id).expect("ERR_NO_BOUNTY");
        BountyOutput {
            id,
            bounty: bounty.into(),
        }
    }

    /// Get number of bounties.
    pub fn get_last_bounty_id(&self) -> u64 {
        self.last_bounty_id
    }

    /// Get `limit` of bounties from given index.
    pub fn get_bounties(&self, from_index: u64, limit: u64) -> Vec<BountyOutput> {
        (from_index..std::cmp::min(from_index + limit, self.last_bounty_id))
            .filter_map(|id| {
                self.bounties.get(&id).map(|bounty| BountyOutput {
                    id,
                    bounty: bounty.into(),
                })
            })
            .collect()
    }

    /// Get bounty claims for given user.
    pub fn get_bounty_claims(&self, account_id: AccountId) -> Vec<BountyClaim> {
        self.bounty_claimers.get(&account_id).unwrap_or_default()
    }

    /// Returns number of claims per given bounty.
    pub fn get_bounty_number_of_claims(&self, id: u64) -> u32 {
        self.bounty_claims_count.get(&id).unwrap_or_default()
    }
}