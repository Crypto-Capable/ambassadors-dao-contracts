use std::cmp::min;
use std::collections::{HashMap, HashSet};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]

pub enum RoleKind{
    Everyone,
    Member(U128),
    Group(HashSet<AccountId>),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]

impl RoleKind{
    ///checking match of user to role
    ///add member to group
    ///remove member from group
    ///number of people in that role
    ///roles and number fo peole in group**
}

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
    /// Convert weight or ratio to specific weight given total weight.
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum WeightKind {
    TokenWeight,
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

impl Policy{
    ///add or update role
    ///remove role
    ///update default vote policy
    ///update parameters 
    ///add members to role
    ///remove member from role
    ///get user roles      {This methodolgy can be used to another similar function where we display all the completed tasks of the CA to claim the bounty}
    ///whether a given role can execute a given particular action
    ///return proposal status for given proposal
}
