use near_sdk::{env, near_bindgen};

use super::*;

pub type MiscellaneousPayout = Payout<Miscellaneous>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Miscellaneous {
    ContentCreationBounty { links_to_content: Vec<ResourceLink> },
    CampusSigningMOU,
    CampusAmbassadorBonus { links_to_content: Vec<ResourceLink> },
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
    pub fn add_payout_miscellaneous(&mut self, miscellaneous: PayoutInput<Miscellaneous>) -> u64 {
        // validate input, seems like there is nothing to do here

        // anyone can create this, no permission checks needed

        // add the miscellaneous to Contract.miscellaneous
        let new_id = self.last_miscellaneous_id + 1;
        self.miscellaneous
            .insert(&new_id, &Payout::from(miscellaneous));
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
        let mut miscellaneous = match self.miscellaneous.get(&id) {
            Some(m) => m,
            None => {
                panic!("{}", error::ErrMiscellaneousNotFound);
            }
        };
        self.internal_act_payout(&mut miscellaneous, action, note);
        // TODO: payout amounts not very clear
    }
}
