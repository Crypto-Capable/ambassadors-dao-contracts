//! Contains the Contract struct and its implementation

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::Base58CryptoHash;
use near_sdk::{env, ext_contract, near_bindgen, serde_json::json, sys};
use near_sdk::{AccountId, CryptoHash, PanicOnDefault, Promise};
use std::str::FromStr;

use ran::*;

use members::Members;
use payout::PayoutInput;
use payout::{BountyPayout, MiscellaneousPayout, Payout, ProposalPayout, Referral, ReferralPayout};
use types::{usd_to_balance, Config, ReferralToken, RegistrationResult, USD};

mod amounts;
mod error;
mod members;
mod payout;
mod types;
mod upgrade;
mod validation;
mod vote;

pub mod views;

// TODO: create a proc_macro for generate meta data about the type of information
// that each Payout type needs for creation

#[ext_contract(ext)]
pub trait CrossContract {
    fn get_exchange_rate(&self) -> f64;
    fn make_transfers(&self, transfers: Vec<(AccountId, USD)>, #[callback_unwrap] cur: f64);
}

/// The main contract governing Ambassadors DAO
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct OldContract {
    /// defines the policy of the contract
    pub members: Members,
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
    /// referral tokens hash map
    pub referral_tokens: LookupMap<ReferralToken, AccountId>,
    /// Large blob storage.
    pub blobs: LookupMap<CryptoHash, AccountId>,
}

/// The main contract governing Ambassadors DAO
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    /// defines the policy of the contract
    pub members: Members,
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
    /// referral tokens hash map
    pub referral_tokens: LookupMap<ReferralToken, AccountId>,
    /// Large blob storage.
    pub blobs: LookupMap<CryptoHash, AccountId>,
    /// What oracle is the contract using
    pub oracle: AccountId,
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
        set_seeds(
            env::random_seed()
                .into_iter()
                .fold(0_u64, |acc, x| acc + (x as u64 * x as u64)),
        );
        Self {
            members: Members::from_council(council),
            config: Config::new(name, purpose),
            proposals: LookupMap::new(b"p".to_vec()),
            last_proposal_id: 0,
            bounties: LookupMap::new(b"b".to_vec()),
            last_bounty_id: 0,
            miscellaneous: LookupMap::new(b"m".to_vec()),
            last_miscellaneous_id: 0,
            referrals: LookupMap::new(b"r".to_vec()),
            last_referral_id: 0,
            referral_tokens: LookupMap::new(b"t".to_vec()),
            blobs: LookupMap::new(b"l".to_vec()),
            oracle: Self::get_oracle(),
        }
    }

    #[private]
    pub fn get_oracle() -> AccountId {
        if env::current_account_id().as_str().ends_with(".near") {
            AccountId::from_str("v1.nearacle.near").unwrap()
        } else {
            AccountId::from_str("v1.nearacle.testnet").unwrap()
        }
    }

    /// Should only be called by this contract on migration.
    /// This is NOOP implementation. KEEP IT if you haven't changed contract state.
    /// If you have changed state, you need to implement migration from old state (keep the old struct with different name to deserialize it first).
    /// After migrate goes live on MainNet, return this implementation for next updates.
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        assert_eq!(
            env::signer_account_id(),
            env::current_account_id(),
            "{}",
            error::ERR_NOT_PERMITTED
        );
        let this = env::state_read::<OldContract>().expect(error::ERR_CONTRACT_NOT_INITIALIZED);
        set_seeds(
            env::random_seed()
                .into_iter()
                .fold(0_u64, |acc, x| acc + (x as u64 * x as u64)),
        );
        Self {
            members: this.members,
            config: this.config,
            proposals: this.proposals,
            last_proposal_id: this.last_proposal_id,
            bounties: this.bounties,
            last_bounty_id: this.last_bounty_id,
            miscellaneous: this.miscellaneous,
            last_miscellaneous_id: this.last_miscellaneous_id,
            referrals: this.referrals,
            last_referral_id: this.last_referral_id,
            referral_tokens: this.referral_tokens,
            blobs: this.blobs,
            oracle: Self::get_oracle(),
        }
    }

    #[private]
    pub fn get_exchange_rate(&self) -> Promise {
        Promise::new(self.oracle.clone()).function_call(
            "get_rate".to_string(),
            json!({
                "currency":"NEAR",
            })
            .to_string()
            .into(),
            0_u128,
            env::used_gas() - env::prepaid_gas(),
        )
    }

    #[private]
    pub fn make_transfers(&self, transfers: Vec<(AccountId, USD)>, #[callback_unwrap] rate: f64) {
        for (payee, usd_amount) in transfers {
            Promise::new(payee).transfer(usd_to_balance(usd_amount, rate));
        }
    }

    /// Perform required actions when an ambassador registers
    /// Requires the sender to send a 24 characters long alphanumeric referral token
    pub fn register_ambassador(&mut self, token: Option<String>) -> RegistrationResult {
        let signer = env::signer_account_id();

        match self.members.ambassadors.get(&signer) {
            // if the ambassadors is registered
            // it means this is being called for creating a registration referral
            Some(_) => {
                panic!("ERR_AMBASSADOR_ALREADY_REGISTERED");
            }
            // the ambassador is not registered
            None => {
                // create a referral token for the ambassador
                let ref_token = Self::internal_generate_referral_id();
                // insert the ref token in the referral ids hashmap
                self.referral_tokens.insert(&ref_token, &signer);

                if let Some(t) = token {
                    if let Some(id) = self.referral_tokens.get(&t) {
                        // create a profile
                        let new_id =
                            self.members
                                .add_ambassador(signer.clone(), ref_token.clone(), true);
                        // add payout record
                        self.add_payout_referral(PayoutInput::<Referral> {
                            description: "Ambassador registration referral".to_string(),
                            information: Referral::AmbassadorRegistration {
                                referrer_id: signer,
                                referred_id: id,
                            },
                        });
                        RegistrationResult::SuccessWithReferral(new_id)
                    } else {
                        let new_id =
                            self.members
                                .add_ambassador(signer.clone(), ref_token.clone(), false);
                        RegistrationResult::SuccessWithoutReferral(
                            new_id,
                            "Your referral token was invalid".to_string(),
                        )
                    }
                } else {
                    // create a profile
                    let new_id =
                        self.members
                            .add_ambassador(signer.clone(), ref_token.clone(), false);
                    RegistrationResult::SuccessWithoutReferral(
                        new_id,
                        "You did not use a referral token".to_string(),
                    )
                }
            }
        }
    }

    /// Remove blob from contract storage and pay back to original storer.
    /// Only original storer can call this.
    pub fn remove_blob(&mut self, hash: Base58CryptoHash) -> Promise {
        let hash: CryptoHash = hash.into();
        // store blobs hash on the contract
        let account_id = self.blobs.remove(&hash).expect("ERR_NO_BLOB");
        assert_eq!(
            env::predecessor_account_id(),
            account_id,
            "ERR_INVALID_CALLER"
        );
        env::storage_remove(&hash);
        let blob_len = env::register_len(u64::MAX - 1).unwrap();
        let storage_cost = ((blob_len + 32) as u128) * env::storage_byte_cost();
        Promise::new(account_id).transfer(storage_cost)
    }
}

#[no_mangle]
pub extern "C" fn store_blob() {
    env::setup_panic_hook();
    let mut contract: Contract = env::state_read().expect(error::ERR_CONTRACT_NOT_INITIALIZED);
    unsafe {
        // Load input into register 0.
        sys::input(0);
        // Compute sha256 hash of register 0 and store in 1.
        sys::sha256(u64::MAX as _, 0_u64, 1);
        // Check if such blob already stored.
        assert_eq!(
            sys::storage_has_key(u64::MAX as _, 1_u64),
            0,
            "ERR_ALREADY_EXISTS"
        );
        // Get length of the input argument and check that enough $NEAR has been attached.
        let blob_len = sys::register_len(0);
        let storage_cost = ((blob_len + 32) as u128) * env::storage_byte_cost();
        assert!(
            env::attached_deposit() >= storage_cost,
            "ERR_NOT_ENOUGH_DEPOSIT:{}",
            storage_cost
        );
        // Store value of register 0 into key = register 1.
        sys::storage_write(u64::MAX as _, 1_u64, u64::MAX as _, 0_u64, 2);
        // Load register 1 into blob_hash and save into LookupMap.
        let blob_hash = [0u8; 32];
        sys::read_register(1, blob_hash.as_ptr() as _);
        contract
            .blobs
            .insert(&blob_hash, &env::predecessor_account_id());
        // Return from function value of register 1.
        let blob_hash_str = near_sdk::serde_json::to_string(&Base58CryptoHash::from(blob_hash))
            .unwrap()
            .into_bytes();
        sys::value_return(blob_hash_str.len() as _, blob_hash_str.as_ptr() as _);
    }
    env::state_write(&contract);
}

impl Contract {
    /// Generate a 24 characters long referral ID.
    /// It contains [a-zA-Z0-9] mcharacters
    pub fn internal_generate_referral_id() -> ReferralToken {
        let mut id_vec = vec![0; 24];
        let ru8 = Rnum::newu8();
        for item in id_vec.iter_mut().take(24) {
            *item = match ru8.rannum_in(0., 9.) {
                Rnum::U8(v) => {
                    if v > 4 {
                        match ru8.rannum_in(97., 122.) {
                            Rnum::U8(n) => n,
                            _ => panic!("{}", error::ERR_GENERATING_RANDOM_NUMBER),
                        }
                    } else {
                        match ru8.rannum_in(65., 90.) {
                            Rnum::U8(n) => n,
                            _ => panic!("{}", error::ERR_GENERATING_RANDOM_NUMBER),
                        }
                    }
                }
                _ => panic!("{}", error::ERR_GENERATING_RANDOM_NUMBER),
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
