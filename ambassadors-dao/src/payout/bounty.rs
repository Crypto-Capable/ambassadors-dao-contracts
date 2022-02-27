use near_sdk::{env, near_bindgen};

use super::*;

pub type BountyPayout = Payout<Bounty>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Bounty {
    HackathonCompletion {
        num_of_registrations: u64,
        num_of_submissions: u64,
        // order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    MemeContestCompletion {
        num_of_submissions: u64,
        // order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    Webinar {
        num_of_registrations: u64,
        num_of_attendees: u64,
        webinar_link: ResourceLink,
    },
    ContentCoordniation {
        content_links: Vec<ResourceLink>,
        story: String,
        tools_used: Vec<String>,
    },
}

impl From<PayoutInput<Bounty>> for Payout<Bounty> {
    fn from(input: PayoutInput<Bounty>) -> Self {
        Self {
            proposer: env::predecessor_account_id(),
            description: input.description,
            info: input.information,
            status: PayoutStatus::UnderConsideration,
            votes_count: 0,
            votes: HashMap::default(),
        }
    }
}

// proposal related function implementation
#[near_bindgen]
impl Contract {
    /// create a bounty payout
    pub fn add_payout_bounty(&mut self, bounty: PayoutInput<Bounty>) -> u64 {
        // validate input, seems like there is nothing to do here

        // anyone can create this, no permission checks needed

        // 3. add the bounty to Contract.bountys
        let new_id = self.last_bounty_id + 1;
        self.bounties.insert(&new_id, &Payout::from(bounty));
        self.last_bounty_id = new_id;
        new_id
    }
    /// act on a bounty payout
    pub fn act_payout_bounty(&mut self, id: u64, action: String, note: Option<String>) {}
}
