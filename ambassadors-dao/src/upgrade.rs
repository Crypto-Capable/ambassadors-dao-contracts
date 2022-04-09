//! Logic to upgrade Sputnik contracts.
use near_sdk::Gas;
use near_sdk::{env, sys};

/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(30_000_000_000_000);

#[allow(dead_code)]
/// Self upgrade, optimizes gas by not loading into memory the code.
/// Accepts the storage hash of the WASM blob
pub(crate) fn upgrade_self(hash: &[u8]) {
    let current_id = env::current_account_id();
    let method_name = "migrate".as_bytes().to_vec();
    let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE_SELF_DEPLOY;
    unsafe {
        // Load input (wasm code) into register 0.
        sys::storage_read(hash.len() as _, hash.as_ptr() as _, 0);
        // schedule a Promise tx to this same contract
        let promise_id = sys::promise_batch_create(
            current_id.as_bytes().len() as _,
            current_id.as_bytes().as_ptr() as _,
        );
        // 1st item in the Tx: "deploy contract" (code is taken from register 0)
        sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
        // 2nd item in the Tx: call this_contract.migrate() with remaining gas
        sys::promise_batch_action_function_call(
            promise_id,
            method_name.len() as _,
            method_name.as_ptr() as _,
            0_u64,
            0_u64,
            0_u64,
            attached_gas.0,
        );
    }
}
