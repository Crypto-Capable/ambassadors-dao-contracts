use crate::*;




#[near_bindgen]
impl Contract {
    /// Returns the semver of the contract
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}
