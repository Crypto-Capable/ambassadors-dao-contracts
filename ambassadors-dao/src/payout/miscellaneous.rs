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
        let extras = match self.miscellaneous.get(&id){
            Some(m) => m,
            None => {
                panic!("ERR_MISC_PAYOUT_NOT_FOUND");
            }
        };
        match action {
            types::Action::RemovePayout => {
                if env::signer_account_id() = extras.proposer{
                    extras.status = PayoutStatus::Removed(note);
                }
                else {
                    panic!("ACTION_NOT_PERMITTED");
                }
            }
            types::Action::VoteApprove => {
                if !self.policy.is_council_member(&env::signer_account_id()){
                    panic!("ERR_ACTION_NOT_PERMITTED");
                }
                extras.votes.insert(env::signer_account_id, vote::Vote::from(action));
                extras.votes_count.approve_count += 1;
            }
            types::Action::VoteReject => {
                if !self.policy.is_council_member(&env::signer_account_id()){
                    panic!("ERR_ACTION_NOT_PERMITTED");
                }
                extras.votes_count.reject_count += 1;
                extras.votes.insert(env::signer_account_id(), vote:Vote::from(action));
            }
        }
    }
}