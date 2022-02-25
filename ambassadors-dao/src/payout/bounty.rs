use near_sdk::{env, near_bindgen};

use super::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum BountyKind {
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

impl From<PayoutInput<BountyKind>> for Payout {
    fn from(input: PayoutInput<BountyKind>) -> Self {
        Self {
            proposer: env::predecessor_account_id(),
            description: input.description,
            info: PayoutInfo::Bounty(input.information),
            status: PayoutStatus::InProgress,
            votes_count: 0,
            votes: HashMap::default(),
        }
    }
}

// proposal related function implementation
#[near_bindgen]
impl Contract {
    /// create a bounty payout
    pub fn add_payout_bounty(&mut self, bounty: PayoutInput<BountyKind>) -> u64 {
        // 1. validate bounty
        // seems like there is nothing to do here

        // 2. check permission of the caller to add this type of a bounty
        // waiting for permissions, roles and actions from @shreyas

        // 3. add the bounty to Contract.bountys
        let new_id = self.last_bounty_id + 1;
        self.bounties.insert(&new_id, &Payout::from(bounty));
        self.last_bounty_id = new_id;
        new_id
    }
    /// act on a bounty payout
    pub fn act_payout_bounty(&mut self, id: u64, action: String, note: Option<String>) {}
}
