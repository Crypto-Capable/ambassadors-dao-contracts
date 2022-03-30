use crate::types::ONE_NEAR;
use near_sdk::{env, near_bindgen};

use super::*;

pub type MiscellaneousPayout = Payout<Miscellaneous>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Miscellaneous {
    ContentCreationBounty {
        /// links to the content peices created
        links_to_content: Vec<ResourceLink>,
        /// the amount that the proposer is expecting to receive
        expected_amount: u64,
        /// a note/brief-description for this bounty
        note: String,
    },
    CampusSigningMOU {
        /// link to the supporting document
        supporting_document: ResourceLink,
    },
    CampusAmbassadorBonus {
        /// links to all the payouts showing credentials for getting a bonus
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
        // validate input
        match &payout.information {
            Miscellaneous::ContentCreationBounty {
                links_to_content, ..
            } => {
                if links_to_content.is_empty() {
                    panic!("ERR_INVALID_LINKS_TO_CONTENT")
                }
            }
            Miscellaneous::CampusSigningMOU {
                supporting_document,
                ..
            } => {
                if supporting_document.trim().len() == 0
                    || !supporting_document.starts_with("https://")
                {
                    panic!("ERR_INVALID_SUPPORTING_DOCUMENT")
                }
            }
            Miscellaneous::CampusAmbassadorBonus { links_to_payouts } => {
                if links_to_payouts.len() == 0 {
                    panic!("ERR_INVALID_LINKS_TO_PAYOUTS")
                }
            }
        };

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
            self.members.is_council_member(&env::signer_account_id()),
            self.members.get_council_size() as u64,
            &mut misc,
            action,
            note,
        );
        self.miscellaneous.insert(&id, &misc);
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
                Miscellaneous::CampusSigningMOU { .. } => {
                    // here the constant is in yoctonear
                    Promise::new(misc.proposer).transfer(amounts::CAMPUS_MOU_AMOUNT);
                }
            };
        }
    }
}
