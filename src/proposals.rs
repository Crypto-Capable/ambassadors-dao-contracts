///impl contract, expected value but found string line 255,256,258,269,271,284,285,296,297,309,319,320
///struct proposal , expected type but found variant, line 89,94,96,97,98,99
///matching proposal inside impl contract , expected value but found &, line 248, 252,263, 267,276,290,302,314



use std::collections::HashMap;
use std::time::SystemTime;
use near_sdk::{near_bindgen, env};
use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::{log, AccountId, Balance, Gas, PromiseOrValue};

use crate::policy::UserInfo;
use crate::types::{Action, Config, GAS_FOR_FT_TRANSFER, ONE_YOCTO_NEAR};
use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus{
    Approved,
    Rejected,
    Removed,
    Expired,
}

/*
pub enum WinnerWallets{
    winnerwal : String,
    runnerwal : String,
    loserwal : String,
}

pub enum WinnerSubmissions{
    winnersub : String,
    runnersub : String,
    losersub : String,
}
*/
pub enum ProposalInfoKind{
    HackathonInfo{
        expected_registration : U128,
        colleges : String,
        marketing_plan : String,
        budget_required : U128,
        themes_tracks : String,
    },
    MemeContestInfo{
        initial_response : String,
        participant_number : U128,
        meme_genre : String,
    },
    HackathonCompletedInfo{
        registration_no: U128,
        team_nos: U128,
        submission_nos: U128,
        winner_submissions: String,
        winner_wallets: String,
    },
    MemeContestCompletedInfo{
        memes_no: U128,
        winner_submissions: String,
        winner_wallets: String,
    },
    WebinarConduct{
        registration_no: U128,
        attended_no: U128,
        webinar_link: String,
    },
    ContentCoordinationInput{
        links: String,
        descriptive_story: String,
    },
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind{
    ChangeConfig{ config:Config },
    AddMemberToRole{ member_id: AccountId, role: String },
    RemoveMemberFromRole { member_id: AccountId, role: String },
    Hackathon {
        created_at: U64::from<env::block_timestamp()>,
        info: ProposalInfoKind::HackathonInfo,
    },

    MemeContest{
        created_at : U64::from<env::block_timestamp()>,
        info : ProposalInfoKind::MemeContestInfo,
    },
    HackathonCompleted{ info: ProposalInfoKind::HackathonCompletedInfo },
    MemeContestCompleted{ info: ProposalInfoKind::MemeContestCompletedInfo },
    Webinar{ info: ProposalInfoKind::WebinarConduct },
    ContentCoordination{ info: ProposalInfoKind::ContentCoordinationInput },
    Vote,
}

impl ProposalKind{
    pub fn to_policy_label(&self) -> &str{
        match self{
            ProposalKind::ChangeConfig { .. } => "config",
            ProposalKind::ChangePolicy { .. } => "policy",
            ProposalKind::AddMemberToRole { .. } => "add_member_to_role",
            ProposalKind::RemoveMemberFromRole { .. } => "remove_member_from_role",
            ProposalKind::Hackathon { .. } => "hackathon",
            ProposalKind::MemeContest { .. } => "meme contest",
            ProposalKind::HackathonCompleted{ .. } => "hackathon_completed",
            ProposalKind::MemeContestCompleted{ .. } => "meme_Contest_Completed",
            ProposalKind::Webinar{ .. } => "webinar",
            ProposalKind:: ContentCoordination{ .. } => "content_coordination",
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Vote {
    Approve = 0x0,
    Reject = 0x1,
    Remove = 0x2,
}

impl From<Action> for Vote {
    fn from(action: Action) -> Self {
        match action {
            Action::VoteApprove => Vote::Approve,
            Action::VoteReject => Vote::Reject,
            Action::VoteRemove => Vote::Remove,
            _ => unreachable!(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal{
    pub status: ProposalStatus,
    pub proposer: AccountId,
    pub kind: ProposalKind,
    pub description: String,
    pub votes: HashMap<AccountId,Vote>,
    pub vote_counts: HashMap<String, [Balance; 3]>,
    pub amount: U64,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum VersionedProposal {
    Default(Proposal),
}

impl From<VersionedProposal> for Proposal {
    fn from(v: VersionedProposal) -> Self {
        match v {
            VersionedProposal::Default(p) => p,
        }
    }
}

impl Proposal{
    pub fn update_votes(
        &mut self,
        account_id: &AccountId,
        roles: &[String],
        vote: Vote,
        policy: &Policy,
        user_weight: Balance,
    ){
        for role in roles {
            self.vote_counts.entry(role.clone()).or_insert([0u128; 3])[vote.clone() as usize] += 1;
        }
        assert!(
            self.votes.insert(account_id.clone(), vote).is_none(),
            "ERR_ALREADY_VOTED"
        );
    }
}

impl Contract{
    pub(crate) fn internal_payout(
        &mut self,
        token_id: &Option<AccountId>,
        receiver_id: &AccountId,
        amount: Balance,
        memo: String,
        msg: Option<String>,
    ) -> PromiseOrValue<()> {
        if token_id.is_none() {
            Promise::new(receiver_id.clone()).transfer(amount).into()
        } else {
            if let Some(msg) = msg {
                ext_fungible_token::ft_transfer_call(
                    receiver_id.clone(),
                    U128(amount),
                    Some(memo),
                    msg,
                    token_id.as_ref().unwrap().clone(),
                    ONE_YOCTO_NEAR,
                    GAS_FOR_FT_TRANSFER,
                )
            } else {
                ext_fungible_token::ft_transfer(
                    receiver_id.clone(),
                    U128(amount),
                    Some(memo),
                    token_id.as_ref().unwrap().clone(),
                    ONE_YOCTO_NEAR,
                    GAS_FOR_FT_TRANSFER,
                )
            }
            .into()
        }
    }
    fn internal_execute_proposal(
        &mut self,
        policy: &Policy,
        proposal: &Proposal,
        proposal_id: u64,
    ) -> PromiseOrValue<()> {
        let result = match &proposal.kind {
            ProposalKind::ChangeConfig { config } => {
                self.config.set(config);
                PromiseOrValue::Value(())
            }
            ProposalKind::ChangePolicy { policy } => {
                self.policy.set(policy);
                PromiseOrValue::Value(())
            }
            ProposalKind::AddMemberToRole { member_id, role } => {
                let mut new_policy = policy.clone();
                new_policy.add_member_to_role(role, &member_id.clone().into());
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::RemoveMemberFromRole { member_id, role } => {
                let mut new_policy = policy.clone();
                new_policy.remove_member_from_role(role, &member_id.clone().into());
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::Hackathon{&mut self, info : &ProposalInfoKind:: HackathonInfo} => {
                let id = self.last_proposal_id;
                self.proposals.insert(
                    Promise::new(ProposalKind::Hackathon {
                        create_at : U64::from<env::block_timestamp()>,
                        info: ProposalInfoKind::HackathonInfo{
                            expected_registration: U128,
                            colleges: String,
                            marketing_plan: String,
                            budget_required: U128,
                            themes_tracks: String,
                        },
                        })
                    )
            }
            ProposalKind::MemeContest{&mut self, info : &ProposalInfoKind::MemeContestInfo} => {
                let id = self.last_proposal_id;
                self.proposals.insert(
                    Promise::new(ProposalKind::MemeContest {
                        create_at : U64::from<env::block_timestamp()>,
                        info: ProposalInfoKind::MemeContestInfo{
                            initial_response: String,
                            participant_number: U128,
                            meme_genre: String,
                        },
                        })
                    )
            }
            ProposalKind::HackathonCompleted{&mut self, info: &ProposalInfoKind::HackathonCompletedInfo} => {
                let id = self.last_proposal_id;
                self.proposals.insert(
                    Promise::new(ProposalKind::HackathonCompletedInfo {
                        info: ProposalInfoKind::HackathonCompletedInfo{
                            registration_no: U128,
                            team_nos: U128,
                            submission_nos: U128,
                            winner_submissions: String,
                            winner_wallets: String,
                        },
                        })
                    )
            }
            ProposalKind::MemeContestCompleted{&mut self, info : &ProposalInfoKind::MemeContestCompletedInfo} => {
                let id = self.last_proposal_id;
                self.proposals.insert(
                    Promise::new(ProposalKind::MemeContestCompleted {
                        info: ProposalInfoKind::MemeContestCompletedInfo{
                            memes_no: U128,
                            winner_submissions: String,
                            winner_wallets: String,
                        },
                        })
                    )
            }
            ProposalKind::Webinar{&mut self, info: &ProposalInfoKin::WebinarConduct} => {
                let id = self.last_proposal_id;
                self.proposals.insert(
                    Promise::new(ProposalKind::Webinar {
                        info: ProposalInfoKind::WebinarConduct{
                            registration_no: U128,
                            attended_no: U128,
                            webinar_link: String,
                        },
                        })
                    )
            }
            ProposalKind::ContentCoordination{&mut self, info: &ProposalInfoKind::ContentCoordinationInput} => {
                let id = self.last_proposal_id;
                self.proposals.insert(
                    Promise::new(ProposalKind::ContentCoordination {
                        info: ProposalInfoKind::ContentCoordinationInput{
                            links: String,
                            descriptive_story: String,
                        },
                        })
                    )
            }
            
        };  
        match result {
            PromiseOrValue::Promise(promise) => promise
                .then(ext_self::on_proposal_callback(
                    proposal_id,
                    env::current_account_id(),
                    0,
                    GAS_FOR_FT_TRANSFER,
                ))
                .into(),
            PromiseOrValue::Value(()) => self.internal_return_bonds(&policy, &proposal).into(),
        }
    }
    pub(crate) fn internal_callback_proposal_fail(
        &mut self,
        proposal: &mut Proposal,
    ) -> PromiseOrValue<()> {
        proposal.status = ProposalStatus::Failed;
        PromiseOrValue::Value(())
    }
    pub(crate) fn internal_user_info(&self) -> UserInfo {
        let account_id = env::predecessor_account_id();
        UserInfo {
            amount: self.get_user_weight(&account_id),
            account_id,
        }
    }
}

#[near_bindgen]
impl Contract{
    #[payable]
    pub fn add_proposal(&mut self, proposal: Proposal::description) -> u64{
        match &proposal.kind {
            ProposalKind::ChangePolicy { policy } => match policy {
                VersionedPolicy::Current(_) => {}
                _ => panic!("ERR_INVALID_POLICY"),
            },
            ProposalKind::AddMemberToRole{ member_id, role } => match Group{
                RoleKind::Group(accounts) => {
                    if accounts.contains(member_id){
                        panic!("ERR_ACCOUNT_EXISTS");
                    }
                }
            },
            ProposalKind::RemoveMemberFromRole{ member_id } => match Group{
                RoleKind::Group(accounts) => {
                    if !accounts.contains(member_id){
                        panic!("ACC_DOESNT_EXISTS");
                    }
                }
            }
        }
    assert!(
        policy
            .can_execute_action(
                self.internal_user_info(),
                &proposal.kind,
                &Action::AddProposal
            )
            .1,
        "ERR_PERMISSION_DENIED"
    );
    let id = self.last_proposal_id;
    self.proposals
        .insert(&id, &VersionedProposal::Default(proposal.into()));
    self.last_proposal_id += 1;
    self.locked_amount += env::attached_deposit();
    id
    }
    pub fn act_proposal(&mut self, id: u64, action: Action, memo: Option<String>) {
        let mut proposal: Proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL").into();
        let policy = self.policy.get().unwrap().to_policy();
        let (roles, allowed) =
            policy.can_execute_action(self.internal_user_info(), &proposal.kind, &action);
        assert!(allowed, "ERR_PERMISSION_DENIED");
        let sender_id = env::predecessor_account_id();
        let update = match action {
            Action::AddProposal => env::panic_str("ERR_WRONG_ACTION"),
            Action::RemoveProposal => {
                self.proposals.remove(&id);
                false
            }

            Action::VoteApprove | Action::VoteReject | Action::VoteRemove => {
                assert!(
                    matches!(proposal.status, ProposalStatus::InProgress),
                    "ERR_PROPOSAL_NOT_READY_FOR_VOTE"
                );
                proposal.update_votes(
                    &sender_id,
                    &roles,
                    Vote::from(action),
                    &policy,
                    self.get_user_weight(&sender_id),
                );
                proposal.status =
                    policy.proposal_status(&proposal, roles, self.total_delegation_amount);
                if proposal.status == ProposalStatus::Approved {
                    self.internal_execute_proposal(&policy, &proposal, id);
                    true
                }
                else{ proposal.status == ProposalStatus::Rejected {
                    self.internal_reject_proposal(&policy, &proposal, id);
                    true
                }}
            }
            Action::Finalize => {
                proposal.status = policy.proposal_status(
                    &proposal,
                    policy.roles.iter().map(|r| r.name.clone()).collect(),
                    self.total_delegation_amount,
                );
                match proposal.status {
                    ProposalStatus::Approved => {
                        self.internal_execute_proposal(&policy, &proposal, id);
                    }
                    _ => {
                        env::panic_str("ERR_PROPOSAL_NOT_EXPIRED_OR_FAILED");
                    }
                }
                true
            }


        };
        if update {
            self.proposals
                .insert(&id, &VersionedProposal::Default(proposal));
        }
        if let Some(memo) = memo {
            log!("Memo: {}", memo);
        }
    }
    #[private]
    pub fn on_proposal_callback(&mut self, proposal_id: u64) -> PromiseOrValue<()> {
        let mut proposal: Proposal = self
            .proposals
            .get(&proposal_id)
            .expect("ERR_NO_PROPOSAL")
            .into();
        assert_eq!(
            env::promise_results_count(),
            1,
            "ERR_UNEXPECTED_CALLBACK_PROMISES"
        );
        let result = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => self.internal_callback_proposal_success(&mut proposal),
            PromiseResult::Failed => self.internal_callback_proposal_fail(&mut proposal),
        };
        self.proposals
            .insert(&proposal_id, &VersionedProposal::Default(proposal.into()));
        result
    }
}