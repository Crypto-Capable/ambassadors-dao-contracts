///default_policy, threshold not in scope , line 161, 322,334,336
///trait bound rolepermission of near_sdk not saisfied, line 136 - cleared


use std::cmp::min;
use std::collections::{HashMap, HashSet};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Balance};

use crate::proposals::{Proposal, ProposalKind, ProposalStatus, Vote};
use crate::types::Action;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]

pub enum RoleKind{
    Everyone,
    Member(U128),
    Group(HashSet<AccountId>),
}

impl RoleKind {
    pub fn match_user(&self, user: &UserInfo) -> bool {
        match self {
            RoleKind::Everyone => true,
            RoleKind::Member(amount) => user.amount >= amount.0,
            RoleKind::Group(accounts) => accounts.contains(&user.account_id),
        }
    }

    pub fn get_role_size(&self) -> Option<usize> {
        match self {
            RoleKind::Group(accounts) => Some(accounts.len()),
            _ => None,
        }
    }

    pub fn add_member_to_group(&mut self, member_id: &AccountId) -> Result<(), ()> {
        match self {
            RoleKind::Group(accounts) => {
                accounts.insert(member_id.clone());
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn remove_member_from_group(&mut self, member_id: &AccountId) -> Result<(), ()> {
        match self {
            RoleKind::Group(accounts) => {
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
    pub vote_policy : HashSet<String, VotePolicy>,
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

///Config of Voting
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct VotePolicy{
    pub weight_kind : WeightKind,
    pub house : U128, 
    pub threshold : WeightOrRatio,
}

impl Default for VotePolicy {
    fn default() -> Self {
        VotePolicy {
            weight_kind: WeightKind::RoleWeight,
            quorum: U128(0),
            threshold: WeightOrRatio::Ratio(1, 2),
        }
    }
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub roles: Vec<RolePermission>,
    pub default_vote_policy: VotePolicy,
    pub proposal_bond: U128,
    pub proposal_period: U64,
    pub bounty_bond: U128,
    pub bounty_forgiveness_period: U64,
}

/// Versioned policy.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum VersionedPolicy {
    Default(Vec<AccountId>),
    Current(Policy),
}


///Default Policy:
///     - everyone can add proposals
///     - group consisting of the call can do all actions, consists of caller.
///     - non token weighted voting, requires 1/2 of the group to vote
///     - proposal & bounty bond is 1N
///     - proposal & bounty forgiveness period is 1 day

impl VersionedPolicy {
    /// Upgrades either version of policy into the latest.
    pub fn upgrade(self) -> Self {
        match self {
            VersionedPolicy::Default(accounts) => {
                VersionedPolicy::Current(default_policy(accounts))
            }
            VersionedPolicy::Current(policy) => VersionedPolicy::Current(policy),
        }
    }

    /// Return recent version of policy.
    pub fn to_policy(self) -> Policy {
        match self {
            VersionedPolicy::Current(policy) => policy,
            _ => unimplemented!(),
        }
    }

    pub fn to_policy_mut(&mut self) -> &mut Policy {
        match self {
            VersionedPolicy::Current(policy) => policy,
            _ => unimplemented!(),
        }
    }
}

impl Policy {
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

    pub fn remove_role(&mut self, role: &String) {
        for i in 0..self.roles.len() {
            if &self.roles[i].name == role {
                self.roles.remove(i);
                return;
            }
        }
        env::log_str(&format!("ERR_ROLE_NOT_FOUND:{}", role));
    }

    pub fn update_default_vote_policy(&mut self, vote_policy: &VotePolicy) {
        self.default_vote_policy = vote_policy.clone();
        env::log_str("Successfully updated the default vote policy.");
    }

    pub fn add_member_to_role(&mut self, role: &String, member_id: &AccountId) {
        for i in 0..self.roles.len() {
            if &self.roles[i].name == role {
                self.roles[i]
                    .kind
                    .add_member_to_group(member_id)
                    .unwrap_or_else(|()| {
                        env::log_str(&format!("ERR_ROLE_WRONG_KIND:{}", role));
                    });
                return;
            }
        }
        env::log_str(&format!("ERR_ROLE_NOT_FOUND:{}", role));
    }

    pub fn remove_member_from_role(&mut self, role: &String, member_id: &AccountId) {
        for i in 0..self.roles.len() {
            if &self.roles[i].name == role {
                self.roles[i]
                    .kind
                    .remove_member_from_group(member_id)
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

    /// Can given user execute given action on this proposal.
    /// Returns all roles that allow this action.
    pub fn can_execute_action(
        &self,
        user: UserInfo,
        proposal_kind: &ProposalKind,
        action: &Action,
    ) -> (Vec<String>, bool) {
        let roles = self.get_user_roles(user);
        let mut allowed = false;
        let allowed_roles = roles
            .into_iter()
            .filter_map(|(role, permissions)| {
                let allowed_role = permissions.contains(&format!(
                    "{}:{}",
                    proposal_kind.to_policy_label(),
                    action.to_policy_label()
                )) || permissions
                    .contains(&format!("{}:*", proposal_kind.to_policy_label()))
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

    fn internal_get_role(&self, name: &String) -> Option<&RolePermission> {
        for role in self.roles.iter() {
            if role.name == *name {
                return Some(role);
            }
        }
        None
    }


    pub fn proposal_status(
        &self,
        proposal: &Proposal,
        roles: Vec<String>,
        total_supply: Balance,
    ) -> ProposalStatus {
        assert!(
            matches!(
                proposal.status,
                ProposalStatus::InProgress | ProposalStatus::Failed
            ),
            "ERR_PROPOSAL_NOT_IN_PROGRESS"
        );
        if proposal.submission_time.0 + self.proposal_period.0 < env::block_timestamp() {
            return ProposalStatus::Expired;
        };
        for role in roles {
            let role_info = self.internal_get_role(&role).expect("ERR_MISSING_ROLE");
            let vote_policy = role_info
                .vote_policy
                .get(&proposal.kind.to_policy_label().to_string())
                .unwrap_or(&self.default_vote_policy);
            let total_weight = match &role_info.kind {
                RoleKind::Everyone => continue,
                RoleKind::Group(group) => {
                    if vote_policy.weight_kind == WeightKind::RoleWeight {
                        group.len() as Balance
                    } else {
                        total_supply
                    }
                }
                RoleKind::Member(_) => total_supply,
            };

            let vote_counts = proposal.vote_counts.get(&role).unwrap_or(&[0u128; 3]);
            if vote_counts[Vote::Approve as usize] >= threshold {
                return ProposalStatus::Approved;
            } else if vote_counts[Vote::Reject as usize] >= threshold {
                return ProposalStatus::Rejected;
            } else if vote_counts[Vote::Remove as usize] >= threshold {
                return ProposalStatus::Removed;
            } else {
            }
        }
        proposal.status.clone()
    }
}
