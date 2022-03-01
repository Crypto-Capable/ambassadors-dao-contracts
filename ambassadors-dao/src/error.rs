use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    NotPermitted,
    ProposalNotFound,
    BountyNotFound,
    MiscellaneousNotFound,
    ReferralNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Error::NotPermitted => "ERR_NOT_PERMITTED",
                Error::ProposalNotFound => "ERR_PROPOSAL_NOT_FOUND",
                Error::BountyNotFound => "ERR_BOUNTY_NOT_FOUND",
                Error::MiscellaneousNotFound => "ERR_MISCELLANEOUS_NOT_FOUND",
                Error::ReferralNotFound => "ERR_REFERRAL_NOT_FOUND",
            }
        )
    }
}
