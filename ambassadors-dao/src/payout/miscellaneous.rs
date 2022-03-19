use crate::types::ONE_NEAR;
use near_sdk::{env, near_bindgen};

use super::*;

pub type MiscellaneousPayout = Payout<Miscellaneous>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Miscellaneous {
    ContentCreationBounty {
        links_to_content: Vec<ResourceLink>,
        expected_amount: u64,
        note: String,
    },
    CampusSigningMOU,
    CampusAmbassadorBonus {
        links_to_payouts: Vec<ResourceLink>,
    },
}

impl From<PayoutInput<Miscellaneous>> for Payout<Miscellaneous> {
    fn from(input: PayoutInput<Miscellaneous>) -> Self {
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
    /// create a miscellaneous payout
    pub fn add_payout_miscellaneous(&mut self, payout: PayoutInput<Miscellaneous>) -> u64 {
        // validate input, seems like there is nothing to do here

        // anyone can create this, no permission checks needed

        // add the miscellaneous to Contract.miscellaneous
        let new_id = self.last_miscellaneous_id + 1;
        self.miscellaneous.insert(&new_id, &Payout::from(payout));
        self.last_miscellaneous_id = new_id;
        new_id
    }
    /// act on a miscellaneous payout
    pub fn act_payout_miscellaneous(
        &mut self,
        id: u64,
        action: types::Action,
        note: Option<String>,
    ) {
        let mut misc = match self.miscellaneous.get(&id) {
            Some(m) => m,
            None => {
                panic!("{}", error::ERR_MISCELLANEOUS_NOT_FOUND);
            }
        };
        internal_act_payout(
            self.policy.is_council_member(&env::signer_account_id()),
            self.policy.get_council_size() as u64,
            &mut misc,
            action,
            note,
        );
        if misc.status == PayoutStatus::Approved {
            match misc.info {
                Miscellaneous::ContentCreationBounty {
                    expected_amount, ..
                } => {
                    // here expected amount is in near
                    Promise::new(misc.proposer).transfer((expected_amount as u128) * ONE_NEAR);
                }
                Miscellaneous::CampusAmbassadorBonus { .. } => {
                    // here the constant is in yoctonear
                    Promise::new(misc.proposer).transfer(amounts::CA_BONUS_AMOUNT);
                }
                Miscellaneous::CampusSigningMOU => {}
            };
        }
    }
}
