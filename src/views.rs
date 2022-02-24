///cannot infer type for struct proposals:Proposals, line 8


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
}