use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

use factory_manager::FactoryManager;

mod factory_manager;

type Version = (u8, u8, u8);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct DaoContractMetadata {
    /// version of the DAO contract code (e.g. (2,1,3) -> 2.1.3, [3,6,0] -> 3.6.0)
    pub version: Version,
    /// commit id of https://github.com/near-daos/sputnik-dao-contract
    /// representing a snapshot of the code that generated the wasm
    pub commit_id: String,
    /// if available, url to the changelog to see the changes introduced in this version
    pub changelog_url: Option<String>,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct SputnikDAOFactory {
    factory_manager: FactoryManager,
    dao: Option<AccountId>,
}

#[near_bindgen]
impl SputnikDAOFactory {
    #[init]
    pub fn new() -> Self {
        Self {
            factory_manager: FactoryManager {},
            dao: None,
        }
    }
}
