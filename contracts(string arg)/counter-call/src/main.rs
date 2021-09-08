#![no_main]

extern crate alloc;

use casper_types::{
    bytesrepr::ToBytes, runtime_args, runtime_args::RuntimeArgs, ApiError, ContractHash, Key,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

const COUNTER_KEY: &str = "counter";
const COUNTER_INC: &str = "counter_inc";
const COUNTER_GET: &str = "counter_get";

#[no_mangle]
pub extern "C" fn call() {
    // Read the Counter smart contract's ContactHash.
    let contract_hash = {
        let counter_uref = runtime::get_key(COUNTER_KEY).unwrap_or_revert_with(ApiError::GetKey);
        if let Key::Hash(hash) = counter_uref {
            ContractHash::new(hash)
        } else {
            runtime::revert(ApiError::User(66));
        }
    };

    let mut test = RuntimeArgs::new();

    let _ = test.insert("hello", "world");

    // Call Counter to increment the value.
    let _: () = runtime::call_contract(contract_hash, COUNTER_INC, test);
}
