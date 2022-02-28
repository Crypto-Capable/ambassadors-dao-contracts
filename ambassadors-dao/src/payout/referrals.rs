use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use super::*;

// create a random number and assign to campus ambassador
pub fn generate_randon_no() -> String{
    let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();
}





#[near_bindgen]
impl Contract {
    pub fn check_random_number(&self, ref_id : generate_randon_no()) {
        // check the input ref id is associated with any campus ambassador
        // if yes, return the id of the campus ambassador
        //then tranfer tokens to the campus ambasador
    }






    // create a referral
    // vote on a referral
    pub fn add_payout_referral(&mut self) {}
}
