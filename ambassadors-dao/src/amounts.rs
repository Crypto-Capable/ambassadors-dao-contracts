use near_sdk::Balance;

use crate::types::ONE_NEAR;
use crate::*;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Amount(pub u128);

impl From<String> for Amount {
    fn from(input: String) -> Self {
        Self(input.parse::<u128>().expect("ERR_SERIALIZING_INTEGER"))
    }
}

impl Into<Balance> for Amount {
    fn into(self) -> Balance {
        self.0
    }
}

impl std::ops::Mul<Amount> for Amount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Mul<u128> for Amount {
    type Output = Self;

    fn mul(self, rhs: u128) -> Self {
        Self(self.0 * rhs)
    }
}

impl std::ops::Div<Amount> for Amount {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::Div<u128> for Amount {
    type Output = Self;

    fn div(self, rhs: u128) -> Self {
        Self(self.0 / rhs)
    }
}

/// 4 near tokens
pub const HACKATHON_COMPLETION_AMOUNT: Amount = Amount(ONE_NEAR * 4);
/// 2 near tokens
pub const MEME_CONTEST_COMPLETION_AMOUNT: Amount = Amount(ONE_NEAR * 2);
/// 2 near tokens
pub const WEBINAR_COMPLETION_AMOUNT: Amount = Amount(ONE_NEAR * 2);
/// 2.5 near tokens
pub const CONTENT_COORDINATION_AMOUNT: Amount = Amount((ONE_NEAR / 10) * 25);

/// 0.5 near tokens
pub const CA_REGISTER_REFERRAL_AMOUNT: Amount = Amount((ONE_NEAR / 10) * 5);

/// 15 near tokens
pub const HACKATHON_FIRST_PLACE_AMOUNT: Amount = Amount(ONE_NEAR * 15);
/// 10 near tokens
pub const HACKATHON_SECOND_PLACE_AMOUNT: Amount = Amount(ONE_NEAR * 10);
/// 5 near tokens
pub const HACKATHON_THIRD_PLACE_AMOUNT: Amount = Amount(ONE_NEAR * 5);

/// 7.5 near tokens
pub const MEME_CONTEST_FIRST_PLACE_AMOUNT: Amount = Amount((ONE_NEAR / 10) * 75);
/// 5 near tokens
pub const MEME_CONTEST_SECOND_PLACE_AMOUNT: Amount = Amount(ONE_NEAR * 5);
/// 2.5 near tokens
pub const MEME_CONTEST_THIRD_PLACE_AMOUNT: Amount = Amount((ONE_NEAR / 10) * 25);

/// 12.5 near tokens
pub const CA_BONUS_AMOUNT: Amount = Amount((ONE_NEAR / 10) * 125);
