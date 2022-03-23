use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};

use super::*;

pub type ReferralPayout = Payout<Referral>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum NCDReferralKind {
    // form filled get $1 per referral
    FormFilled,
    // ncd course completion get $5 per referral
    Completion,
}

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
    Recruitment {
        /// the account ID (person) that reffered a new recruitee
        referrer_id: AccountId,
        /// the account ID (person) that was referred to
        referred_id: AccountId,
    },
    NearCertifiedDeveloper {
        /// the account ID (person) that reffered a new recruitee
        referrer_id: AccountId,
        /// the account ID (person) that was referred to
        referred_id: AccountId,
        /// the kind of ncd referral
        kind: NCDReferralKind,
        /// a link to the proof
        proof_link: ResourceLink,
    },
}

impl From<PayoutInput<Referral>> for Payout<Referral> {
    fn from(input: PayoutInput<Referral>) -> Self {
        Self {
            proposer: env::predecessor_account_id(),
            description: input.description,
            info: input.information,
            status: PayoutStatus::UnderConsideration,
            votes_count: VotesCount::new(),
            votes: HashMap::default(),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn add_payout_referral(&mut self, payout: PayoutInput<Referral>) -> u64 {
        // validate input, seems like there is nothing to do here

        // anyone can create this, no permission checks needed

        // add the referral to Contract.referrals
        let new_id = self.last_referral_id + 1;
        self.referrals.insert(&new_id, &Payout::from(payout));
        self.last_referral_id = new_id;
        new_id
    }
    /// act on a proposal payout
    pub fn act_payout_referral(&mut self, id: u64, action: types::Action, note: Option<String>) {
        // check if proposal with id exists
        let mut referral = match self.referrals.get(&id) {
            Some(p) => p,
            None => panic!("{}", error::ERR_PROPOSAL_NOT_FOUND),
        };
        // if proposal is not under consideration, it is final
        match referral.status {
            PayoutStatus::UnderConsideration => {}
            _ => panic!("{}: {}", error::ERR_NOT_PERMITTED, "payout finalized"),
        }
        internal_act_payout(
            self.policy.is_council_member(&env::signer_account_id()),
            self.policy.get_council_size() as u64,
            &mut referral,
            action,
            note,
        );
        // check if payout state is approved
        if referral.status == PayoutStatus::Approved {
            // here tokens is in near value
            let (payee, amount) = match referral.info {
                Referral::AmbassadorRegistration { referred_id, .. } => {
                    (referred_id, amounts::CA_REGISTER_REFERRAL_AMOUNT)
                }
                Referral::Recruitment { referred_id, .. } => {
                    (referred_id, amounts::RECRUITMENT_REFERRAL_AMOUNT)
                }
                Referral::NearCertifiedDeveloper {
                    referred_id, kind, ..
                } => (
                    referred_id,
                    match kind {
                        NCDReferralKind::Completion => amounts::NCD_COMPLETION_REFERRAL_AMOUNT,
                        NCDReferralKind::FormFilled => amounts::NCD_FORM_FILLED_REFERRAL_AMOUNT,
                    },
                ),
            };
            Promise::new(payee).transfer(amount);
        }
    }
}
