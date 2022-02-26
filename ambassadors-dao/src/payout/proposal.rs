use near_sdk::{env, near_bindgen};

use super::*;

pub type ProposalPayout = Payout<Proposal>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Proposal {
    Hackathon {
        expected_registrations: u64,
        estimated_budget: u64,
        supporting_document: ResourceLink,
    },
    MemeContest {
        expected_registrations: u64,
        estimated_budget: u64,
        supporting_document: ResourceLink,
    },
    Open {
        supporting_document: ResourceLink,
    },
}

impl From<PayoutInput<Proposal>> for Payout<Proposal> {
    fn from(input: PayoutInput<Proposal>) -> Self {
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
    /// create a proposal payout
    pub fn add_payout_proposal(&mut self, proposal: PayoutInput<Proposal>) -> u64 {
        // 1. validate proposal
        // seems like there is nothing to do here

        // 2. check permission of the caller to add this type of a proposal
        // waiting for permissions, roles and actions from @shreyas

        // 3. add the proposal to Contract.proposals
        let new_id = self.last_proposal_id + 1;
        self.proposals.insert(&new_id, &Payout::from(proposal));
        self.last_proposal_id = new_id;
        new_id
    }
    /// act on a proposal payout
    pub fn act_payout_proposal(&mut self, id: u64, action: String, note: Option<String>) {}
}
