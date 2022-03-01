use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;

use super::*;

struct Refferal {
    connect: HashMap<AccountId, String>,
}

#[near_bindgen]
impl Contract {
    pub fn generate_randon_no(&mut self) -> String {}

    pub fn validate_ref_id(&self, ref_id: String) {
        // check the input ref id is associated with any campus ambassador
        // if yes, return the id of the campus ambassador

        //then tranfer tokens to the campus ambasador
    }

    // create a referral
    // vote on a referral
    pub fn add_payout_referral(&mut self) {}
}
