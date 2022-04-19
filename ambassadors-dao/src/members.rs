use std::collections::{HashMap, HashSet};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct AmbassadorProfile {
    pub id: u64,
    pub referral_token: types::ReferralToken,
    pub registration_referral_used: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Members {
    /// council-member-account-id
    pub council: HashSet<AccountId>,
    // the id of the last ambassador
    pub last_ambassador_id: u64,
    /// <ambassador-account-id, ambassador-referral-token>
    pub ambassadors: HashMap<AccountId, AmbassadorProfile>,
}

impl Members {
    /// create a new members struct
    pub fn new() -> Self {
        Members {
            council: HashSet::new(),
            last_ambassador_id: 0,
            ambassadors: HashMap::new(),
        }
    }

    /// creates a new members struct with given council ids and referral tokens
    pub fn from_council(input: Vec<AccountId>) -> Self {
        Members {
            council: HashSet::from_iter(input.into_iter()),
            last_ambassador_id: 0,
            ambassadors: HashMap::new(),
        }
    }

    /// if the given account ID is a member of the council
    pub fn is_council_member(&self, account_id: &AccountId) -> bool {
        self.council.contains(account_id)
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
    pub fn add_ambassador(
        &mut self,
        account_id: AccountId,
        referral_token: types::ReferralToken,
        registration_referral_used: bool,
    ) -> u64 {
        let id = self.last_ambassador_id + 1;
        self.ambassadors.insert(
            account_id,
            AmbassadorProfile {
                id,
                referral_token,
                registration_referral_used,
            },
        );
        self.last_ambassador_id = id;
        id
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
        self.members.council.iter().cloned().collect()
    }

    /// Returns the referral token of the council members
    /// Can only be accessed a council member or smart contract account
    pub fn get_ambassador_profile(&self, account_id: AccountId) -> AmbassadorProfile {
        self.members
            .ambassadors
            .get(&account_id)
            .expect(error::ERR_AMBASSADOR_NOT_FOUND)
            .clone()
    }
}
