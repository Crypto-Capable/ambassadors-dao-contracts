use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;
use near_sdk::AccountId;

use super::*;

struct Refferal{
     ambassador : AccountId,
     ref_id : String,
     connect : HashMap<ambassador, ref_id>,
}

impl Refferal{
// create a random number and assign to campus ambassador
    pub fn generate_randon_no(&mut self) -> String{
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
        self.connect.insert(ambassador, rand_string);
        return rand_string.to_string();
    }
}


#[near_bindgen]
impl Contract {
    pub fn validate_ref_id(&self, ref_id : generate_randon_no()) {
        // check the input ref id is associated with any campus ambassador
        
        // if yes, return the id of the campus ambassador

        //then tranfer tokens to the campus ambasador
    }






    // create a referral
    // vote on a referral
    pub fn add_payout_referral(&mut self) {}
}
