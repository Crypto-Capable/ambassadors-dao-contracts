use crate::upgrade::internal_get_factory_info;
use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub council: HashMap<AccountId, String>,
    pub ambassadors: HashMap<AccountId, String>,
}

impl Policy {
    /// create a new empty policy
    pub fn new() -> Self {
        Policy {
            council: HashMap::new(),
            ambassadors: HashMap::new(),
        }
    }
    /// if the given account ID is a member of the council
    pub fn is_council_member(&self, member_id: &AccountId) -> bool {
        self.council.contains_key(member_id)
    }
    /// get size of council   
    pub fn get_council_size(&self) -> usize {
        self.council.len()
    }
    /// add member to council
    /// can only be done by a council member
    /// NOTE: not adding support for this rn
    pub fn add_member_to_council(&mut self, member_id: &AccountId, referral_token: String) {
        self.council.insert(member_id.clone(), referral_token);
    }
    /// remove member from council
    /// can only be done by a council member
    /// NOTE: not adding support for this rn
    pub fn remove_member_from_council(&mut self, member_id: &AccountId) {
        self.council.remove(member_id);
    }
    /// if the given account ID is a member of the council
    pub fn is_registered_ambassador(&self, member_id: &AccountId) -> bool {
        self.ambassadors.contains_key(member_id)
    }
}

impl Default for Policy {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<(AccountId, String)>> for Policy {
    fn from(input: Vec<(AccountId, String)>) -> Self {
        let mut council = HashMap::with_capacity(input.len());
        for info in input {
            council.insert(info.0, info.1);
        }
        Policy {
            council: council,
            ambassadors: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Returns the referral token of the council members
    /// Can only be accessed by the factory or a council member
    pub fn get_council_referral_tokens(&self) -> HashMap<AccountId, String> {
        let signer = env::signer_account_id();
        if self.policy.council.contains_key(&signer)
            || internal_get_factory_info().factory_id == signer
        {
            self.policy.council.clone()
        } else {
            panic!("{}", error::ERR_NOT_PERMITTED)
        }
    }

    /// Returns the referral token of the council members
    /// Can only be accessed by council or factory
    pub fn get_council_referral_token(&self, account_id: AccountId) -> String {
        let signer = env::signer_account_id();
        if self.policy.council.contains_key(&signer)
            || internal_get_factory_info().factory_id == signer
        {
            self.policy
                .council
                .get(&account_id)
                .expect(error::ERR_REFERRAL_TOKEN_NOT_FOUND)
                .into()
        } else {
            panic!("{}", error::ERR_NOT_PERMITTED)
        }
    }

    /// Returns the referral token of the account_id
    /// Can only be accessed by council or factory or account owner
    pub fn get_ambassador_referral_token(&self, account_id: AccountId) -> String {
        let signer = env::signer_account_id();
        if signer == account_id
            || self.policy.is_council_member(&signer)
            || signer == internal_get_factory_info().factory_id
        {
            self.policy
                .ambassadors
                .get(&account_id)
                .expect(error::ERR_REFERRAL_TOKEN_NOT_FOUND)
                .into()
        } else {
            panic!("{}", error::ERR_NOT_PERMITTED)
        }
    }
}
