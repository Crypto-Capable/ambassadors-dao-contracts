use near_sdk::Balance;

use crate::types::ONE_NEAR;

/// 4 near tokens
pub const HACKATHON_COMPLETION_AMOUNT: Balance = 4 * ONE_NEAR;
/// 2 near tokens
pub const MEME_CONTEST_COMPLETION_AMOUNT: Balance = 2 * ONE_NEAR;
/// 2 near tokens
pub const WEBINAR_COMPLETION_AMOUNT: Balance = 2 * ONE_NEAR;
/// 2.5 near tokens
pub const CONTENT_COORDINATION_AMOUNT: Balance = 25 * ONE_NEAR / 10;

/// 0.5 near tokens
pub const CA_REGISTER_REFERRAL_AMOUNT: Balance = 5 * ONE_NEAR / 10;

/// 15 near tokens
pub const HACKATHON_FIRST_PLACE_AMOUNT: Balance = 15 * ONE_NEAR;
/// 10 near tokens
pub const HACKATHON_SECOND_PLACE_AMOUNT: Balance = 10 * ONE_NEAR;
/// 5 near tokens
pub const HACKATHON_THIRD_PLACE_AMOUNT: Balance = 5 * ONE_NEAR;

/// 7.5 near tokens
pub const MEME_CONTEST_FIRST_PLACE_AMOUNT: Balance = 75 * ONE_NEAR / 10;
/// 5 near tokens
pub const MEME_CONTEST_SECOND_PLACE_AMOUNT: Balance = 5 * ONE_NEAR;
/// 2.5 near tokens
pub const MEME_CONTEST_THIRD_PLACE_AMOUNT: Balance = 25 * ONE_NEAR / 10;

/// 12.5 near tokens
pub const CA_BONUS_AMOUNT: Balance = 125 * ONE_NEAR / 10;
