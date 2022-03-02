use near_sdk::{env, near_bindgen};

use super::*;

pub type BountyPayout = Payout<Bounty>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Bounty {
    HackathonCompletion {
        /// number of registrations
        num_of_registrations: u64,
        /// number of submissions
        num_of_submissions: u64,
        /// information of the winners
        /// order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    MemeContestCompletion {
        /// number of submissions
        num_of_submissions: u64,
        /// information of the winners
        /// order of the winners w.r.to their ranks
        winners_info: Vec<SubmissionInfo>,
    },
    Webinar {
        /// number of registrations
        num_of_registrations: u64,
        /// number of attendees
        num_of_attendees: u64,
        /// link to the webinar meeting
        webinar_link: ResourceLink,
    },
    ContentCoordniation {
        /// a list of links to the content
        content_links: Vec<ResourceLink>,
        /// a brief summary about the story of creation of the content
        story: String,
        /// a list of the name of tools used
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
            votes_count: VotesCount::new(),
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

        // add the bounty to Contract.bountys
        let new_id = self.last_bounty_id + 1;
        self.bounties.insert(&new_id, &Payout::from(bounty));
        self.last_bounty_id = new_id;
        new_id
    }
    /// act on a bounty payout
    pub fn act_payout_bounty(&mut self, id: u64, action: types::Action, note: Option<String>) {
        let mut bounty = match self.bounties.get(&id) {
            Some(b) => b,
            None => {
                panic!("{}", error::ErrBountyNotFound);
            }
        };
        self.internal_act_payout(&mut bounty, action, note);
        // check if payout state is approved
        if bounty.status == PayoutStatus::Approved {
            let tokens = match bounty.info {
                Bounty::HackathonCompletion { .. } => amounts::HackathonCompletionAmount,
                Bounty::MemeContestCompletion { .. } => amounts::MemeContestCompletionAmount,
                Bounty::Webinar { .. } => amounts::WebinarCompletionAmount,
                Bounty::ContentCoordniation { .. } => amounts::ContentCoordinationAmount,
            };
            Promise::new(bounty.proposer).transfer(tokens);
        }
    }
}
