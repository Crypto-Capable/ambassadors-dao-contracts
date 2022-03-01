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

        // 3. add the bounty to Contract.bountys
        let new_id = self.last_bounty_id + 1;
        self.bounties.insert(&new_id, &Payout::from(bounty));
        self.last_bounty_id = new_id;
        new_id
    }
    /// act on a bounty payout
    pub fn act_payout_bounty(&mut self, id: u64, action: types::Action, note: Option<String>) {

        // check if bounty exists
        // let the council vote: 
        // if approved, vote count will be increased by 1
        // if rejected , vote count will be decresaed by 1,
        // if removed, it will be removed with a note as to why not worthy

        let mut bounty = match self.bounties.get(&id){
            // match the id and checking if such a bounty exists and returning error or existance
            Some(b) => b;
            None => {
                panic!("NO_SUCH_BOUNTY_EXISTS");
            }
        };
        // match the environment signer, give him different vote options, basis on counil or not
        match action {
            types::Action::RemovePayout => {
                if env::signer_account_id() == bounty.proposer{
                    bounty.status = PayoutStatus::Removed(note);
                }
                else { 
                    panic!("ACTION_NOT_PERMITTED");
                }
            }
            types::Action::VoteApprove => {
                if !self.policy.is_council_member(&env::signer_account_id()){
                    panic!("NOT_PERMITTED");
                }
                bounty.votes.insert(env::signer_account_id, vote::Vote::from(action));
                bounty.votes_count.approve_count += 1;
            }
            types::Action::VoteReject => {
                if !self.policy.is_council_member(&env::signer_account_id()){
                    panic!("NOT_PERMITTED");
                }
                bounty.votes_count.reject_count += 1;
                bounty.votes.insert(env::signer_account_id(), vote:Vote::from(action));
            }
        }
    }
}
