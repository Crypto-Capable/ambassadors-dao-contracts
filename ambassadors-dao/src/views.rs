// implement multiple proposal based on proposer


use std::cmp::min;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};

use payout::{Bounty, Proposal, Miscellaneous};

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
    // paginated view
    // if env signer is council: then they can have paginated view of payouts
    // if env signer is not council: then they will have only payouts they have added


    pub fn get_proposals(&self, start_index: u64, limit: u64) -> Vec<PayoutOutput<Proposal>>{
        let signer = env::signer_account_id();
        if self.policy.is_council_member(&signer){
            (start_index..min(self.last_proposal_id, start_index+limit)).filter_map(|id|{
                self.proposals.get(&id).map(|p| PayoutOutput{id, payout: p})
            }).collect()
        }
        // check
        else {
            (start_index..min(self.last_proposal_id, start_index+limit)).filter_map(|id|{
                self.proposals.get(&id).map(|p| PayoutOutput{id, payout: p})
            }).collect()
        }
    }
    pub fn get_bounties(&self, start_index: u64, limit: u64) -> Vec<PayoutOutput<Bounty>>{
        if self.policy.is_council_member(&env::signer_account_id()){
            (start_index..min(self.last_bounty_id, start_index+limit)).filter_map(|id|{
                self.bounties.get(&id).map(|b| PayoutOutput{id, payout: b})
            }).collect()
        }
        // check
        else{
            (start_index..min(self.last_bounty_id, start_index+limit)).filter_map(|id|{
                self.bounties.get(&id).map(|b| PayoutOutput{id, payout: b})
            }).collect()
        }
    }
        pub fn get_miscellaneous(&self, start_index:u64, limit:u64) -> Vec<PayoutOutput<Miscellaneous>>{
            if self.policy.is_council_member(&env::signer_account_id()){
            (start_index..min(self.last_miscellaneous_id, start_index+limit)).filter_map(|id|{
                self.miscellaneous.get(&id).map(|m| PayoutOutput{id, payout: m})
            }).collect()
        } 
        // check
        else{
            (start_index..min(self.last_miscellaneous_id, start_index+limit)).filter_map(|id|{
                self.miscellaneous.get(&id).map(|m| PayoutOutput{id, payout: m})
            }).collect()
        }
    }

    pub fn get_proposal(&self, id: u64) -> PayoutOutput<Proposal>{
        let proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
        let signer = env::signer_account_id();
        if self.policy.is_council_member(&signer)|| signer == proposal.proposer{
        PayoutOutput {
            id,
            payout: proposal,
            }
        } // need check
        else{
            PayoutOutput{
                id,
                payout: proposal,
            }
        }
    }

    pub fn get_bounty(&self, id: u64) -> PayoutOutput<Bounty>{
        let bounty = self.bounties.get(&id).expect("NO_BOUNTY_FOUND");
        let signer = env::signer_account_id();
        if self.policy.is_council_member(&signer)|| signer == bounty.proposer{
        PayoutOutput {
            id,
            payout: bounty,
            }
        } // need check
        else{
            PayoutOutput{
                id,
                payout : bounty,
            }
        }
    }

    pub fn get_miscellaneous_by_id(&self, id: u64) -> PayoutOutput<Miscellaneous>{
        let miscellaneous = self.miscellaneous.get(&id).expect("NO_MISC_PAYOUT_FOUND");
        let signer = env::signer_account_id();
        if self.policy.is_council_member(&env::signer_account_id()) || signer == miscellaneous.proposer{
        PayoutOutput {
            id,
            payout: miscellaneous,
            }
        } // need check
        else{
            PayoutOutput {
                id,
                payout: miscellaneous,
                }
        }
    }

}








// paginated view
// if env signer is council: then they can have paginated view of payouts
// if env signer is not council: then they will have only payouts they have added

// get diff payout by id
// if env signer is council: can view it if it exists
// if env signer is campus ambassador: can view only if it is added by him. 