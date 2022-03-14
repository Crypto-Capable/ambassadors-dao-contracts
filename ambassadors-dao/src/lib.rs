//! Contains the Contract struct and its implementation
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use near_sdk::{AccountId, PanicOnDefault, Promise};
use rand::distributions::{Alphanumeric, DistString};
// use std::sync::{Arc, Mutex};
// use std::thread;

use payout::{
    BountyPayout, MiscellaneousPayout, Payout, PayoutInput, ProposalPayout, Referral,
    ReferralPayout,
};
use policy::Policy;
use types::Config;
use upgrade::internal_set_factory_info;
use upgrade::FactoryInfo;

mod amounts;
mod error;
mod payout;
mod policy;
mod types;
mod upgrade;
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
    /// referral payouts
    referrals: LookupMap<u64, ReferralPayout>,
    /// the id of the last referral
    last_referral_id: u64,
    /// store the referral ids as a map of <referral-id, account-id>
    referral_ids: LookupMap<String, AccountId>,
    // store the current USD conversion rate, conversion_rate == 1 Near token
    // conversion_rate: Option<f32>,
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
        if params.council.is_empty() {
            panic!("ERR_COUNCIL_EMPTY");
        }
        if params.name.is_empty() {
            panic!("ERR_INVALID_NAME");
        }
        if params.purpose.is_empty() {
            panic!("ERR_PURPOSE_EMPTY");
        }
        let contract = Self {
            policy: Policy::from(params.council.clone()),
            config: Config::new(params.name, params.purpose),
            proposals: LookupMap::<u64, ProposalPayout>::new(b"p".to_vec()),
            last_proposal_id: 0,
            bounties: LookupMap::<u64, BountyPayout>::new(b"b".to_vec()),
            last_bounty_id: 0,
            miscellaneous: LookupMap::<u64, MiscellaneousPayout>::new(b"m".to_vec()),
            last_miscellaneous_id: 0,
            referrals: LookupMap::<u64, ReferralPayout>::new(b"r".to_vec()),
            last_referral_id: 0,
            referral_ids: {
                let mut map = LookupMap::new(b"t".to_vec());
                map.extend(
                    params
                        .council
                        .iter()
                        .map(|id| (Self::internal_generate_referral_id(), id.clone())),
                );
                map
            },
            // conversion_rate: val,
        };
        internal_set_factory_info(&FactoryInfo {
            factory_id: env::predecessor_account_id(),
            auto_update: true,
        });
        contract
    }

    /// Generate a 16 characters long referral ID.
    /// It contains [a-zA-Z0-9] mcharacters
    #[private]
    pub fn internal_generate_referral_id() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    }

    /// Perform required actions when an ambassador registers
    pub fn register_ambassador(&mut self, token: Option<String>) -> String {
        // create a referral token for the new ambassador
        let ref_token = Self::internal_generate_referral_id();
        let signer = env::signer_account_id();
        self.referral_ids.insert(&ref_token, &signer);

        // check if there was a referral token used by the new ambassador
        if let Some(token) = token {
            if let Some(id) = self.referral_ids.get(&token) {
                // add payout record
                self.add_payout_referral(PayoutInput::<Referral> {
                    description: "Registration referral payout, pre-approved by DAO".to_string(),
                    information: Referral::AmbassadorRegistration {
                        referrer_id: signer,
                        referred_id: id.clone(),
                    },
                });
                // transfer the referral reward
                Promise::new(id).transfer(amounts::CA_REGISTER_REFERRAL_AMOUNT);
            }
        }

        ref_token
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_contract() {
        println!("it works");
    }
}
