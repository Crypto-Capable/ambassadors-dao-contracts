use near_sdk::Balance;

use crate::types::ONE_NEAR;

/// 4 near tokens
pub const HACKATHON_COMPLETION_AMOUNT: Balance = ONE_NEAR * 4;
/// 2 near tokens
pub const MEME_CONTEST_COMPLETION_AMOUNT: Balance = ONE_NEAR * 2;
/// 2 near tokens
pub const WEBINAR_COMPLETION_AMOUNT: Balance = ONE_NEAR * 2;
/// 2.5 near tokens
pub const CONTENT_COORDINATION_AMOUNT: Balance = (ONE_NEAR / 10) * 25;

/// 0.5 near tokens
pub const CA_REGISTER_REFERRAL_AMOUNT: Balance = (ONE_NEAR / 10) * 5;

/// 0.1 near tokens
pub const RECRUITMENT_REFERRAL_AMOUNT: Balance = ONE_NEAR;

/// 15 near tokens
pub const HACKATHON_FIRST_PLACE_AMOUNT: Balance = ONE_NEAR * 15;
/// 10 near tokens
pub const HACKATHON_SECOND_PLACE_AMOUNT: Balance = ONE_NEAR * 10;
/// 5 near tokens
pub const HACKATHON_THIRD_PLACE_AMOUNT: Balance = ONE_NEAR * 5;

/// 7.5 near tokens
pub const MEME_CONTEST_FIRST_PLACE_AMOUNT: Balance = (ONE_NEAR / 10) * 75;
/// 5 near tokens
pub const MEME_CONTEST_SECOND_PLACE_AMOUNT: Balance = ONE_NEAR * 5;
/// 2.5 near tokens
pub const MEME_CONTEST_THIRD_PLACE_AMOUNT: Balance = (ONE_NEAR / 10) * 25;

/// 0.5 near tokens
pub const NCD_COMPLETION_REFERRAL_AMOUNT: Balance = (ONE_NEAR / 10) * 5;
/// 0.1 near tokens
pub const NCD_FORM_FILLED_REFERRAL_AMOUNT: Balance = ONE_NEAR / 10;

/// 12.5 near tokens
pub const CA_BONUS_AMOUNT: Balance = (ONE_NEAR / 10) * 125;

/// 10 near tokens
pub const CAMPUS_MOU_AMOUNT: Balance = 10 * ONE_NEAR;
