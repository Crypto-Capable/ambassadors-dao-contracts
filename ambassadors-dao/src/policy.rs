use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

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
