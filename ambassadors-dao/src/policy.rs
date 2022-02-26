use std::collections::HashSet;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub council: HashSet<AccountId>,
}

impl Policy {
    /// create a new empty policy
    pub fn new() -> Self {
        Policy {
            council: HashSet::new(),
        }
    }
    /// if the given account ID is a member of the council
    pub fn is_council_member(&self, member_id: &AccountId) -> bool {
        self.council.contains(member_id)
    }
    /// get size of council   
    pub fn get_council_size(&self) -> usize {
        self.council.len()
    }
    /// add member to council
    /// can only be done by a council member
    /// NOTE: not adding support for this rn
    pub fn add_member_to_council(&mut self, member_id: &AccountId) {
        self.council.insert(member_id.clone());
    }
    /// remove member from council
    /// can only be done by a council member
    /// NOTE: not adding support for this rn
    pub fn remove_member_from_council(&mut self, member_id: &AccountId) {
        self.council.remove(member_id);
    }
}

impl From<Vec<AccountId>> for Policy {
    fn from(input: Vec<AccountId>) -> Self {
        let mut set = HashSet::with_capacity(input.len());
        for id in input {
            set.insert(id);
        }
        Policy { council: set }
    }
}
