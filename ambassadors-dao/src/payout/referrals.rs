use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;

use super::*;

struct Refferal{
     campusambassador : env::predecessor_account_id(),
     ref_id : String,
     connect : HashMap<campusambassador, ref_id>,
}

impl Refferal{
// create a random number and assign to campus ambassador
    pub fn generate_random_no(&mut self) -> String{
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
        self.connect.insert(campusambassador, rand_string);
        return rand_string.to_string();
    }
}


#[near_bindgen]
impl Contract {
    pub fn check_random_number(&self, ref_id : String) {
        // check the input ref id is associated with any campus ambassador

        // if yes, return the id of the campus ambassador

        //then tranfer tokens to the campus ambasador
    }






    // create a referral
    // vote on a referral
    pub fn add_payout_referral(&mut self) {}
}
