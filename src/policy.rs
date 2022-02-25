use std::cmp::min;
use std::collections::{HashMap, HashSet};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Balance};

use crate::types::Action;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]

pub enum RoleKind{
    CampusAmbassador(U128),
    Council(HashSet<AccountId>),
}

impl RoleKind{
    ///given a username and accountid , return the data is true or not
    pub fn match_user(&self, user:&UserInfo) -> bool {
        match self {
            RoleKind::CampusAmbassador(amount)user.amount >= amount.0,
            RoleKind::Council(accounts) => accounts.contains(&user.account_id),
        }
    }
    ///get size of council   
    pub fn get_council_size(&self) -> Option<usize> {
        match self {
            RoleKind::Council(accounts) => Some(accounts.len()),
            _ => None,
        }
    }
    /// add member to council
    pub fn add_member_to_council(&mut self, member_id: &AccountId) -> Result<(), ()> {
        match self {
            RoleKind::Council(accounts) => {
                accounts.insert(member_id.clone());
                Ok(())
            }
            _ => Err(()),
        }
    }
    ///remove member from council
    pub fn remove_member_from_council(&mut self, member_id: &AccountId) -> Result<(), ()> {
        match self {
            RoleKind::Council(accounts) => {
                accounts.remove(member_id);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]

pub struct RolePermission{
    pub name : String,
    pub kind : RoleKind,
    pub permission : HashSet<String>,
}

pub struct UserInfo {
    pub account_id: AccountId,
    pub amount: Balance,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum WeightOrRatio {
    Weight(U128),
    Ratio(u64, u64),
}

impl WeightOrRatio{
    pub fn to_weight(&self, total_weight: Balance) -> Balance {
        match self {
            WeightOrRatio::Ratio(num, denom) => min(
                (*num as u128 * total_weight) / *denom as u128 + 1,
                total_weight,
            ),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum WeightKind {
    RoleWeight,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub roles: Vec<RolePermission>,
}

impl Policy{
    ///To update a role
    pub fn add_or_update_role(&mut self, role: &RolePermission) {
        for i in 0..self.roles.len() {
            if &self.roles[i].name == &role.name {
                env::log_str(&format!(
                    "Updating existing role in the policy:{}",
                    &role.name
                ));
                let _ = std::mem::replace(&mut self.roles[i], role.clone());
                return;
            }
        }
        env::log_str(&format!("Adding new role to the policy:{}", &role.name));
        self.roles.push(role.clone());
    }
    ///remove a role
    pub fn remove_role(&mut self, role: &String) {
        for i in 0..self.roles.len() {
            if &self.roles[i].name == role {
                self.roles.remove(i);
                return;
            }
        }
        env::log_str(&format!("ERR_ROLE_NOT_FOUND:{}", role));
    }
    ///adding a member to a council
    pub fn add_member_to_role(&mut self, role: &String, member_id: &AccountId) {
        for i in 0..self.roles.len() {
            if &self.roles[i].name == role {
                self.roles[i]
                    .kind
                    .add_member_to_council(member_id)
                    .unwrap_or_else(|()| {
                        env::log_str(&format!("ERR_ROLE_WRONG_KIND:{}", role));
                    });
                return;
            }
        }
        env::log_str(&format!("ERR_ROLE_NOT_FOUND:{}", role));
    }
    ///remove a member from council
    pub fn remove_member_from_role(&mut self, role: &String, member_id: &AccountId) {
        for i in 0..self.roles.len() {
            if &self.roles[i].name == role {
                self.roles[i]
                    .kind
                    .remove_member_from_council(member_id)
                    .unwrap_or_else(|()| {
                        env::log_str(&format!("ERR_ROLE_WRONG_KIND:{}", role));
                    });
                return;
            }
        }
        env::log_str(&format!("ERR_ROLE_NOT_FOUND:{}", role));
    }

    /// Returns set of roles that this user is member of permissions for given user across all the roles it's member of.
    fn get_user_roles(&self, user: UserInfo) -> HashMap<String, &HashSet<String>> {
        let mut roles = HashMap::default();
        for role in self.roles.iter() {
            if role.kind.match_user(&user) {
                roles.insert(role.name.clone(), &role.permissions);
            }
        }
        roles
    }
    ///who can execute a certain given action
    pub fn can_execute_action(
        &self,
        user: UserInfo,
        action: &Action,
        payout_kind : &PayoutInfo,
    ) -> (Vec<String>, bool){
        let roles = self.get_user_roles(user);
        let mut allowed = false;
        let allowed_roles = roles
            .into_iter()
            .filter_map(|(role, permissions)| {
                let allowed_role = permissions.contains(&format!(
                    "{}:{}",
                    payout_kind.to_policy_label(),
                    action.to_policy_label()
                )) || permissions
                    .contains(&format!("{}:*", payout_kind.to_policy_label()))
                    || permissions.contains(&format!("*:{}", action.to_policy_label()))
                    || permissions.contains("*:*");
                allowed = allowed || allowed_role;
                if allowed_role {
                    Some(role)
                } else {
                    None
                }
            })
            .collect();
        (allowed_roles, allowed)
    }
    ///get roles by name
    fn internal_get_role(&self, name: &String) -> Option<&RolePermission> {
        for role in self.roles.iter() {
            if role.name == *name {
                return Some(role);
            }
        }
        None
    }
    ///status of payout: either approved, rejected, underconsideration
    pub fn payout_status(
        &self,
        payout: &Payout,
        roles: Vec<String>,
        total_supply: Balance,
    ) -> PayoutStatus{
        assert!(
            matches!(
                payout.status,
                PayoutStatus::UnderConsideration | PayoutStatu::Rejected
            ),
            "ERR_PROPOSAL_NOT_IN_PROGRESS"
        );
        
        for role in roles {
            let role_info = self.internal_get_role(&role).expect("ERR_MISSING_ROLE");
            let total_weight = match &role_info.kind {
                RoleKind::Council(group) => total_supply,
                
                RoleKind::CampusAmbassadors(_) => total_supply,
            };
            let vote_counts = proposal.vote_counts.get(&role).unwrap_or(&[0u128; 3]);
            let  threshold = WeightOrRatio::Ratio(1, 2);
            if vote_counts[Vote::Approve as usize] >= threshold {
                return PayoutStatus::Approved;
            } else if vote_counts[Vote::Reject as usize] >= threshold {
                return PayoutStatus::Rejected;
            } else {
            }
        }
        payout.status.clone()
    }
}