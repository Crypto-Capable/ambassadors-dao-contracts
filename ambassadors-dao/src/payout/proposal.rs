use crate::types::ONE_NEAR;
use near_sdk::{env, near_bindgen};

use super::*;

pub type ProposalPayout = Payout<Proposal>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum Proposal {
    Hackathon {
        /// number of expected registrations in the hackathon
        expected_registrations: u64,
        /// estimated budget required for the hackathon in near tokens
        estimated_budget: u64,
        /// s3 link to a PDF with details of the proposal
        supporting_document: ResourceLink,
    },
    MemeContest {
        /// number of expected registrations in the meme contest
        expected_registrations: u64,
        /// estimated budget required for the meme contest in near tokens
        estimated_budget: u64,
        /// s3 link to a PDF with details of the proposal
        supporting_document: ResourceLink,
    },
    Open {
        /// estimated budget required for the proposal in near tokens
        estimated_budget: u64,
        /// s3 link to a PDF with details of the proposal
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
            votes_count: VotesCount::new(),
            votes: HashMap::default(),
        }
    }
}

#[near_bindgen]
impl Contract {
    /// create a proposal payout
    pub fn add_payout_proposal(&mut self, payout: PayoutInput<Proposal>) -> u64 {
        // validate input
        match &payout.information {
            Proposal::Hackathon {
                supporting_document,
                ..
            } => {
                if supporting_document.trim().len() == 0
                    || !supporting_document.starts_with("https://")
                {
                    panic!("ERR_INVALID_SUPPORTING_DOCUMENT")
                }
            }
            Proposal::MemeContest {
                supporting_document,
                ..
            } => {
                if supporting_document.trim().len() == 0
                    || !supporting_document.starts_with("https://")
                {
                    panic!("ERR_INVALID_SUPPORTING_DOCUMENT")
                }
            }
            Proposal::Open {
                supporting_document,
                ..
            } => {
                if supporting_document.trim().len() == 0
                    || !supporting_document.starts_with("https://")
                {
                    panic!("ERR_INVALID_SUPPORTING_DOCUMENT")
                }
            }
        };

        // anyone can create this, no permission checks needed

        // add the proposal to Contract.proposals
        let new_id = self.last_proposal_id + 1;
        self.proposals.insert(&new_id, &Payout::from(payout));
        self.last_proposal_id = new_id;
        new_id
    }

    /// act on a proposal payout
    pub fn act_payout_proposal(&mut self, id: u64, action: types::Action, note: Option<String>) {
        // check if proposal with id exists
        let mut proposal = match self.proposals.get(&id) {
            Some(p) => p,
            None => panic!("{}", error::ERR_PROPOSAL_NOT_FOUND),
        };
        // if proposal is not under consideration, it is final
        match proposal.status {
            PayoutStatus::UnderConsideration => {}
            _ => panic!("{}: {}", error::ERR_NOT_PERMITTED, "payout finalized"),
        }
        internal_act_payout(
            self.members.is_council_member(&env::signer_account_id()),
            self.members.get_council_size() as u64,
            &mut proposal,
            action,
            note,
        );
        // check if payout state is approved
        if proposal.status == PayoutStatus::Approved {
            // here tokens is in near value
            let tokens = match proposal.info {
                Proposal::Hackathon {
                    estimated_budget, ..
                } => estimated_budget,
                Proposal::MemeContest {
                    estimated_budget, ..
                } => estimated_budget,
                Proposal::Open {
                    estimated_budget, ..
                } => estimated_budget,
            };
            Promise::new(proposal.proposer).transfer((tokens as u128) * ONE_NEAR);
        }
    }
}
