use near_sdk::{env, near_bindgen};

use super::{types::Action, *};

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
        /// number of registrations
        num_of_registrations: u64,
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
    ContentCoordination {
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

#[near_bindgen]
impl Contract {
    /// create a bounty payout
    pub fn add_payout_bounty(&mut self, payout: PayoutInput<Bounty>) -> u64 {
        // validate input
        match &payout.information {
            Bounty::HackathonCompletion {
                num_of_registrations,
                winners_info,
                ..
            } => {
                if *num_of_registrations > 20 {
                    panic!("ERR_MIN_SUBMISSION_LIMIT_NOT_SATISFIED")
                }
                if winners_info.len() != 3 {
                    panic!("ERR_WINNERS_INFO_MUST_HAVE_THREE_ENTRIES")
                }
            }
            Bounty::MemeContestCompletion {
                num_of_registrations,
                winners_info,
                ..
            } => {
                if *num_of_registrations > 20 {
                    panic!("ERR_MIN_SUBMISSION_LIMIT_NOT_SATISFIED")
                }
                if winners_info.len() != 3 {
                    panic!("ERR_WINNERS_INFO_MUST_HAVE_THREE_ENTRIES")
                }
            }
            Bounty::Webinar {
                num_of_attendees,
                webinar_link,
                ..
            } => {
                if *num_of_attendees > 50 {
                    panic!("ERR_MIN_ATTENDEES_LIMIT_NOT_SATISFIED")
                }
                validation::assert_valid_resource_url(webinar_link);
            }
            Bounty::ContentCoordination {
                content_links,
                story,
                tools_used,
            } => {
                if content_links.is_empty() {
                    panic!("ERR_CONTENT_LINKS_CANNOT_BE_EMPTY")
                }
                if story.is_empty() {
                    panic!("ERR_STORY_CANNOT_BE_EMPTY")
                }
                if tools_used.is_empty() {
                    panic!("ERR_TOOLS_USED_CANNOT_BE_EMPTY")
                }
            }
        };
        // anyone can create this, no permission checks needed

        // add the bounty to Contract.bountys
        let new_id = self.last_bounty_id + 1;
        self.bounties.insert(&new_id, &Payout::from(payout));
        self.last_bounty_id = new_id;
        new_id
    }

    /// act on a bounty payout
    pub fn act_payout_bounty(&mut self, id: u64, action: Action, note: Option<String>) {
        let mut bounty = match self.bounties.get(&id) {
            Some(b) => b,
            None => {
                panic!("{}", error::ERR_BOUNTY_NOT_FOUND);
            }
        };
        internal_act_payout(
            self.members.is_council_member(&env::signer_account_id()),
            self.members.get_council_size() as u64,
            &mut bounty,
            action,
            note,
        );
        self.bounties.insert(&id, &bounty);
        // check if payout state is approved
        if bounty.status == PayoutStatus::Approved {
            // send the respective winners tokens
            // here tokens is in yoctonear
            let transfers = match bounty.info {
                Bounty::HackathonCompletion { winners_info, .. } => {
                    vec![
                        (
                            bounty.proposer.clone(),
                            amounts::HACKATHON_COMPLETION_AMOUNT,
                        ),
                        (
                            winners_info[0].account_id.clone(),
                            amounts::HACKATHON_FIRST_PLACE_AMOUNT,
                        ),
                        (
                            winners_info[1].account_id.clone(),
                            amounts::HACKATHON_SECOND_PLACE_AMOUNT,
                        ),
                        (
                            winners_info[2].account_id.clone(),
                            amounts::HACKATHON_THIRD_PLACE_AMOUNT,
                        ),
                    ]
                }
                Bounty::MemeContestCompletion { winners_info, .. } => {
                    vec![
                        (
                            bounty.proposer.clone(),
                            amounts::MEME_CONTEST_COMPLETION_AMOUNT,
                        ),
                        (
                            winners_info[0].account_id.clone(),
                            amounts::MEME_CONTEST_FIRST_PLACE_AMOUNT,
                        ),
                        (
                            winners_info[1].account_id.clone(),
                            amounts::MEME_CONTEST_SECOND_PLACE_AMOUNT,
                        ),
                        (
                            winners_info[2].account_id.clone(),
                            amounts::MEME_CONTEST_THIRD_PLACE_AMOUNT,
                        ),
                    ]
                }
                Bounty::Webinar { .. } => {
                    vec![(bounty.proposer, amounts::WEBINAR_COMPLETION_AMOUNT)]
                }
                Bounty::ContentCoordination { .. } => {
                    vec![(bounty.proposer, amounts::CONTENT_COORDINATION_AMOUNT)]
                }
            };
            // get the exchange rate
            self.get_exchange_rate().then(ext::make_transfers(
                transfers,
                env::current_account_id(),
                0,
                env::used_gas() - env::prepaid_gas(),
            ));
        }
    }
}
