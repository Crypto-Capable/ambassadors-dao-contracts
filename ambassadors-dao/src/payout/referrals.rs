use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};

use super::{types::Action, *};

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
    /// create a new referral payout
    pub fn add_payout_referral(&mut self, payout: PayoutInput<Referral>) -> u64 {
        // validate input
        match &payout.information {
            Referral::AmbassadorRegistration {
                referred_id,
                referrer_id,
            } => {
                if !self.members.is_registered_ambassador(referred_id) {
                    panic!("{}", error::ERR_REFERRED_MEMBER_NOT_FOUND);
                }
                if self
                    .members
                    .ambassadors
                    .get(referrer_id)
                    .unwrap()
                    .registration_referral_used
                {
                    panic!("REGISTRATION_REFERRAL_ALREADY_USED");
                }
            }
            Referral::NearCertifiedDeveloper {
                referred_id,
                proof_link,
                ..
            } => {
                if self.members.is_registered_ambassador(referred_id) {
                    panic!("{}", error::ERR_REFERRED_MEMBER_NOT_FOUND);
                }
                validation::assert_valid_resource_url(proof_link);
            }
            Referral::Recruitment { referred_id, .. } => {
                if self.members.is_registered_ambassador(referred_id) {
                    panic!("{}", error::ERR_REFERRED_MEMBER_NOT_FOUND);
                }
            }
        };

        // anyone can create this, no permission checks needed

        // add the referral to Contract.referrals
        let new_id = self.last_referral_id + 1;
        self.referrals.insert(&new_id, &Payout::from(payout));
        self.last_referral_id = new_id;
        new_id
    }

    // Add a registration referral using a referral token
    pub fn add_registration_referral_with_token(&mut self, token: String) -> u64 {
        let signer = env::signer_account_id();
        let ambassador: &mut members::AmbassadorProfile =
            match self.members.ambassadors.get_mut(&signer) {
                None => {
                    panic!("{}", error::ERR_AMBASSADOR_NOT_FOUND);
                }
                Some(m) => m,
            };
        if ambassador.registration_referral_used {
            panic!("{}", error::ERR_NOT_PERMITTED);
        }
        // lets get the account id to whom the referral token belongs
        match self.referral_tokens.get(&token) {
            // valid referral token
            Some(account_id) => {
                // if the token belongs to the signer
                if account_id == signer {
                    panic!("{}", error::ERR_NOT_PERMITTED);
                } else {
                    ambassador.registration_referral_used = true;
                    self.add_payout_referral(PayoutInput::<Referral> {
                        description: "Ambassador registration referral".to_string(),
                        information: Referral::AmbassadorRegistration {
                            referrer_id: signer,
                            referred_id: account_id,
                        },
                    })
                }
            }
            // invalid referral token
            None => {
                panic!("{}", error::ERR_INVALID_REFERRAL_TOKEN);
            }
        }
    }

    /// act on a referral payout
    pub fn act_payout_referral(&mut self, id: u64, action: Action, note: Option<String>) {
        // check if proposal with id exists
        let mut referral = match self.referrals.get(&id) {
            Some(p) => p,
            None => panic!("{}", error::ERR_PROPOSAL_NOT_FOUND),
        };
        internal_act_payout(
            self.members.is_council_member(&env::signer_account_id()),
            self.members.get_council_size() as u64,
            &mut referral,
            action,
            note,
        );
        self.referrals.insert(&id, &referral);
        // check if payout state is approved
        if referral.status == PayoutStatus::Approved {
            // here tokens is in near value
            let transfer = match referral.info {
                Referral::AmbassadorRegistration {
                    referred_id,
                    referrer_id,
                } => {
                    self.members
                        .ambassadors
                        .get_mut(&referrer_id)
                        .unwrap()
                        .registration_referral_used = true;
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
            self.get_exchange_rate().then(ext::make_transfers(
                vec![transfer],
                env::current_account_id(),
                0,
                env::used_gas() - env::prepaid_gas(),
            ));
        }
    }
}
