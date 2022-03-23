//! Contains the Contract struct and its implementation
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::Base58CryptoHash;
use near_sdk::CryptoHash;
use near_sdk::{env, near_bindgen, sys};
use near_sdk::{AccountId, PanicOnDefault, Promise};
use ran::*;

use payout::{
    BountyPayout, MiscellaneousPayout, Payout, PayoutInput, ProposalPayout, Referral,
    ReferralPayout,
};
use policy::Policy;
use types::Config;

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
    /// Large blob storage.
    pub blobs: LookupMap<CryptoHash, AccountId>,
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
        set_seeds(
            env::random_seed()
                .into_iter()
                .fold(0_u64, |acc, x| acc + (x as u64 * x as u64)),
        );
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
        Self {
            policy: Policy::from(council_info),
            config: Config::new(name, purpose),
            proposals: LookupMap::new(b"p".to_vec()),
            last_proposal_id: 0,
            bounties: LookupMap::new(b"b".to_vec()),
            last_bounty_id: 0,
            miscellaneous: LookupMap::new(b"m".to_vec()),
            last_miscellaneous_id: 0,
            referrals: LookupMap::new(b"r".to_vec()),
            last_referral_id: 0,
            referral_ids: ref_map,
            blobs: LookupMap::new(b"l".to_vec()),
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
        let this: Contract = env::state_read().expect(error::ERR_CONTRACT_NOT_INITIALIZED);
        this
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
            }
        }

        return true;
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
        sys::sha256(u64::MAX as _, 0 as _, 1);
        // Check if such blob already stored.
        assert_eq!(
            sys::storage_has_key(u64::MAX as _, 1 as _),
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
        sys::storage_write(u64::MAX as _, 1 as _, u64::MAX as _, 0 as _, 2);
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
    pub fn internal_generate_referral_id() -> String {
        let mut id_vec = vec![0; 24];
        let ru8 = Rnum::newu8();
        for i in 0..24 {
            id_vec[i] = match ru8.rannum_in(0., 9.) {
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
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    #[test]
    fn generates_contract() {
        println!("it works");
    }

    #[test]
    fn random_number() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let contract = Contract::new(
            "ca-dao".to_string(),
            "dao".to_string(),
            vec![AccountId::new_unchecked("sidtest.testnet".to_string())],
        );
        println!("{}", Contract::internal_generate_referral_id());
        println!("{}", Contract::internal_generate_referral_id());
        println!("{}", Contract::internal_generate_referral_id());
        println!("{}", Contract::internal_generate_referral_id());
    }
}
