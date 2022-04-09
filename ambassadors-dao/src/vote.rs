use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use crate::error::ERR_INVALID_ACTION;
use crate::types::Action;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Vote {
    Approve,
    Reject,
}

impl From<Action> for Vote {
    fn from(input: Action) -> Self {
        match input {
            Action::VoteApprove => Vote::Approve,
            Action::VoteReject => Vote::Reject,
            _ => {
                panic!("{}", ERR_INVALID_ACTION);
            }
        }
    }
}
