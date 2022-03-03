use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};

use super::*;

pub type ReferralPayout = Payout<Referral>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Referral {
    AmbassadorRegistration {
        /// the account ID (person) that used a referral token
        referrer_id: AccountId,
        /// the account ID (person) to which the token belongs
        referred_id: AccountId,
    },
}

impl From<PayoutInput<Referral>> for Payout<Referral> {
    fn from(input: PayoutInput<Referral>) -> Self {
        match input.information {
            // an ambassador referral can only be created inside the smart contract itself
            // it will be Approved by default
            Referral::AmbassadorRegistration { .. } => Self {
                proposer: env::predecessor_account_id(),
                description: input.description,
                info: input.information,
                status: PayoutStatus::Approved,
                votes_count: VotesCount::new(),
                votes: HashMap::default(),
            },
            _ => Self {
                proposer: env::predecessor_account_id(),
                description: input.description,
                info: input.information,
                status: PayoutStatus::UnderConsideration,
                votes_count: VotesCount::new(),
                votes: HashMap::default(),
            },
        }
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn add_payout_referral(&mut self, referral: PayoutInput<Referral>) -> u64 {
        // validate input, seems like there is nothing to do here

        // anyone can create this, no permission checks needed

        // add the referral to Contract.referrals
        let new_id = self.last_referral_id + 1;
        self.referrals.insert(&new_id, &Payout::from(referral));
        self.last_referral_id = new_id;
        new_id
    }
}
