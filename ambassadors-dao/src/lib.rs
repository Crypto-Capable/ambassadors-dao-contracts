//! Contains the Contract struct and its implementation
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use near_sdk::{AccountId, PanicOnDefault, Promise};
use ran::*;

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
    pub policy: Policy,
    /// the configuration of the contract
    pub config: Config,
    /// proposal payouts
    pub proposals: LookupMap<u64, ProposalPayout>,
    /// the id of the last proposal
    pub last_proposal_id: u64,
    /// proposal payouts
    pub bounties: LookupMap<u64, BountyPayout>,
    /// the id of the last proposal
    pub last_bounty_id: u64,
    /// proposal payouts
    pub miscellaneous: LookupMap<u64, MiscellaneousPayout>,
    /// the id of the last proposal
    pub last_miscellaneous_id: u64,
    /// referral payouts
    pub referrals: LookupMap<u64, ReferralPayout>,
    /// the id of the last referral
    pub last_referral_id: u64,
    /// store the referral ids as a map of <referral-id, account-id>
    pub referral_ids: LookupMap<String, AccountId>,
    // store the current USD conversion rate, conversion_rate == 1 Near token
    // pub conversion_rate: Option<f32>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(name: String, purpose: String, council: Vec<AccountId>) -> Self {
        if name.is_empty() {
            panic!("ERR_INVALID_NAME");
        }
        if purpose.is_empty() {
            panic!("ERR_PURPOSE_EMPTY");
        }
        if council.is_empty() {
            panic!("ERR_COUNCIL_EMPTY");
        }
        // generate the referral tokens for the council
        let council_info = council
            .iter()
            .map(|id| (id.clone(), Self::internal_generate_referral_id()))
            .collect::<Vec<_>>();
        // create the referral lookup map
        let mut ref_map = LookupMap::new(b"t".to_vec());
        ref_map.extend(
            council_info
                .iter()
                .map(|(id, token)| (token.clone(), id.clone())),
        );
        // instantiate the contract itself
        let contract = Self {
            policy: Policy::from(council_info),
            config: Config::new(name, purpose),
            proposals: LookupMap::<u64, ProposalPayout>::new(b"p".to_vec()),
            last_proposal_id: 0,
            bounties: LookupMap::<u64, BountyPayout>::new(b"b".to_vec()),
            last_bounty_id: 0,
            miscellaneous: LookupMap::<u64, MiscellaneousPayout>::new(b"m".to_vec()),
            last_miscellaneous_id: 0,
            referrals: LookupMap::<u64, ReferralPayout>::new(b"r".to_vec()),
            last_referral_id: 0,
            referral_ids: ref_map,
            // conversion_rate: val,
        };
        internal_set_factory_info(&FactoryInfo {
            factory_id: env::predecessor_account_id(),
            auto_update: true,
        });
        contract
    }

    /// Perform required actions when an ambassador registers
    /// Requires the sender to send a 24 characters long alphanumeric referral token
    pub fn register_ambassador(&mut self, token: Option<String>) -> bool {
        let signer = env::signer_account_id();
        // if ambassador already exists
        if self.policy.is_registered_ambassador(&signer) {
            return false;
        }
        // create a referral token for the new ambassador
        let ref_token = Self::internal_generate_referral_id();
        // insert it in the policy
        self.policy
            .ambassadors
            .insert(signer.clone(), ref_token.clone());
        // insert the ref token in the referral ids hashmap
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
                Promise::new(id).transfer(amounts::CA_REGISTER_REFERRAL_AMOUNT.into());
            }
        }

        return true;
    }
}

impl Contract {
    /// Generate a 16 characters long referral ID.
    /// It contains [a-zA-Z0-9] mcharacters
    pub fn internal_generate_referral_id() -> String {
        set_seeds(env::block_timestamp());
        let mut id_vec = vec![0; 24];
        let ru8 = Rnum::newu8();
        for i in 0..24 {
            id_vec[i] = match ru8.rannum_in(0., 9.) {
                Rnum::U8(v) => {
                    if v > 4 {
                        match ru8.rannum_in(97., 122.) {
                            Rnum::U8(n) => n,
                            _ => panic!("ERR_GENERATING_RANDOM_NUMBER"),
                        }
                    } else {
                        match ru8.rannum_in(65., 90.) {
                            Rnum::U8(n) => n,
                            _ => panic!("ERR_GENERATING_RANDOM_NUMBER"),
                        }
                    }
                }
                _ => panic!("ERR_GENERATING_RANDOM_NUMBER"),
            };
        }
        String::from_utf8(id_vec).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_contract() {
        println!("it works");
    }
}
