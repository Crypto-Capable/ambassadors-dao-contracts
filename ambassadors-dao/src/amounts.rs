use crate::types;
use near_sdk::Balance;

/// 4 near tokens
pub const HackathonCompletionAmount: Balance = 4 * types::ONE_NEAR;
/// 2 near tokens
pub const MemeContestCompletionAmount: Balance = 2 * types::ONE_NEAR;
/// 2 near tokens
pub const WebinarCompletionAmount: Balance = 2 * types::ONE_NEAR;
/// 2.5 near tokens
pub const ContentCoordinationAmount: Balance = 25 * types::ONE_NEAR / 10;

/// 0.5 near tokens
pub const CARegisterReferralAmount: Balance = 5 * types::ONE_NEAR / 10;

/// 15 near tokens
pub const HackathonFirstPlaceAmount: Balance = 15 * types::ONE_NEAR;
/// 10 near tokens
pub const HackathonSecondPlaceAmount: Balance = 10 * types::ONE_NEAR;
/// 5 near tokens
pub const HackathonThirdPlaceAmount: Balance = 5 * types::ONE_NEAR;

/// 7.5 near tokens
pub const MemeContestFirstPlaceAmount: Balance = 75 * types::ONE_NEAR / 10;
/// 5 near tokens
pub const MemeContestSecondPlaceAmount: Balance = 5 * types::ONE_NEAR;
/// 2.5 near tokens
pub const MemeContestThirdPlaceAmount: Balance = 25 * types::ONE_NEAR / 10;
