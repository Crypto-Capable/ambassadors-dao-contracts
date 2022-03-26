use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Members {
    /// <council-member-account-id, council-member-referral-token>
    pub council: HashMap<AccountId, String>,
    /// <ambassador-account-id, ambassador-referral-token>
    pub ambassadors: HashMap<AccountId, String>,
}

impl Members {
    /// create a new members struct
    pub fn new() -> Self {
        Members {
            council: HashMap::new(),
            ambassadors: HashMap::new(),
        }
    }

    /// creates a new members struct with given council ids and referral tokens
    pub fn from_council(input: Vec<(AccountId, String)>) -> Self {
        let mut council = HashMap::with_capacity(input.len());
        for info in input {
            council.insert(info.0, info.1);
        }
        Members {
            council: council,
            ambassadors: HashMap::new(),
        }
    }

    /// if the given account ID is a member of the council
    pub fn is_council_member(&self, account_id: &AccountId) -> bool {
        self.council.contains_key(account_id)
    }

    /// get size of council   
    pub fn get_council_size(&self) -> usize {
        self.council.len()
    }

    /// if the given account ID is a member of the council
    pub fn is_registered_ambassador(&self, account_id: &AccountId) -> bool {
        self.ambassadors.contains_key(account_id)
    }

    /// add a new member in the ambassadors field
    pub fn add_ambassador(&mut self, account_id: AccountId, token: String) {
        self.ambassadors.insert(account_id, token);
    }
}

impl Default for Members {
    fn default() -> Self {
        Self::new()
    }
}

#[near_bindgen]
impl Contract {
    /// Returns the referral token of the council members
    /// Can only be accessed a council member or smart contract account
    pub fn get_council(&self) -> Vec<AccountId> {
        let signer = env::signer_account_id();
        if self.members.is_council_member(&signer) || signer == env::current_account_id() {
            self.members
                .council
                .iter()
                .map(|(k, _)| k.to_owned())
                .collect()
        } else {
            panic!("{}", error::ERR_NOT_PERMITTED)
        }
    }

    /// Returns the referral token of the council members
    /// Can only be accessed by council or smart contract account
    pub fn get_council_referral_token(&self, account_id: AccountId) -> String {
        let signer = env::signer_account_id();
        if self.members.is_council_member(&signer) || signer == env::current_account_id() {
            self.members
                .council
                .get(&account_id)
                .expect(error::ERR_REFERRAL_TOKEN_NOT_FOUND)
                .into()
        } else {
            panic!("{}", error::ERR_NOT_PERMITTED)
        }
    }

    /// Returns the referral token of the account_id
    /// Can only be accessed by council or account owner or smart contract account
    pub fn get_ambassador_referral_token(&self, account_id: AccountId) -> String {
        let signer = env::signer_account_id();
        if signer == account_id
            || self.members.is_council_member(&signer)
            || signer == env::current_account_id()
        {
            self.members
                .ambassadors
                .get(&account_id)
                .expect(error::ERR_REFERRAL_TOKEN_NOT_FOUND)
                .into()
        } else {
            panic!("{}", error::ERR_NOT_PERMITTED)
        }
    }
}
