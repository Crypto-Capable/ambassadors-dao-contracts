//! Contains the Contract struct and its implementation
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use near_sdk::{AccountId, PanicOnDefault, Promise};
use rand::distributions::{Alphanumeric, DistString};
// use std::sync::{Arc, Mutex};
// use std::thread;

use payout::{BountyPayout, MiscellaneousPayout, Payout, ProposalPayout};
use policy::Policy;
use types::Config;

mod error;
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
    /// defines the policy of the contract
    policy: Policy,
    /// the configuration of the contract
    config: Config,
    /// proposal payouts
    proposals: LookupMap<u64, ProposalPayout>,
    /// the id of the last proposal
    last_proposal_id: u64,
    /// proposal payouts
    bounties: LookupMap<u64, BountyPayout>,
    /// the id of the last proposal
    last_bounty_id: u64,
    /// proposal payouts
    miscellaneous: LookupMap<u64, MiscellaneousPayout>,
    /// the id of the last proposal
    last_miscellaneous_id: u64,
    /// store the referral ids as a map of <referral-id, account-id>
    referral_ids: LookupMap<String, String>,
    // store the current USD conversion rate, conversion_rate == 1 Near token
    // conversion_rate: Arc<Mutex<Option<f32>>>,
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
        if params.council.len() == 0 {
            panic!("ERR_COUNCIL_EMPTY");
        }
        if params.name.len() == 0 {
            panic!("ERR_INVALID_NAME");
        }
        if params.purpose.len() == 0 {
            panic!("ERR_PURPOSE_EMPTY");
        }
        // let conversion_rate = Arc::new(Mutex::new(Option<f32>::None));
        // let conversion_rate_clone = Arc::clone(&conversion_rate);
        // let join_handler = thread::spawn(move || {
        // fetch data and update the conversion rate
        // let agent: Agent = ureq::AgentBuilder::new()
        //     .timeout_read(Duration::from_secs(5))
        //     .timeout_write(Duration::from_secs(5))
        //     .build();
        // loop {
        // let body: String = agent.get("https://helper.mainnet.near.org/fiat")
        //     .call()?
        //     .into_string()?;
        // update on the contract struct
        // thread::sleep(std::time::Duration::from_secs(2));
        // }
        // });
        Self {
            policy: Policy::from(params.council),
            config: Config::new(params.name, params.purpose),
            proposals: LookupMap::<u64, ProposalPayout>::new(b"p".to_vec()),
            last_proposal_id: 0,
            bounties: LookupMap::<u64, BountyPayout>::new(b"b".to_vec()),
            last_bounty_id: 0,
            miscellaneous: LookupMap::<u64, MiscellaneousPayout>::new(b"m".to_vec()),
            last_miscellaneous_id: 0,
            referral_ids: {
                let map = LookupMap::new(b"t".to_vec());
                map.extend(
                    params
                        .council
                        .iter()
                        .map(|id| (Self::internal_generate_referral_id(), id.to_string())),
                );
                map
            },
            // conversion_rate: val,
        }
    }

    /// Generate a 16 characters long referral ID.
    /// It contains [a-zA-Z0-9] mcharacters
    #[private]
    pub fn internal_generate_referral_id() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    }

    /// Perform required actions when an ambassador registers
    pub fn register_ambassador(&mut self, token: Option<String>) -> String {
        // create a referral token
        let ref_token = Self::internal_generate_referral_id();
        self.referral_ids
            .insert(&String::from(env::signer_account_id()), &ref_token);

        // check if there was a token passed
        if let Some(token) = token {
            match self.referral_ids.get(&token) {
                Some(id) => {
                    Promise::new(AccountId::new_unchecked(token))
                        .transfer(5 / 10 * types::ONE_NEAR);
                }
                None => {}
            }
        }

        ref_token
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_contract() {
        unimplemented!()
    }
}
