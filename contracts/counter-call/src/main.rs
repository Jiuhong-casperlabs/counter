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

    // let mut inc_value = RuntimeArgs::new();
    // let _ = inc_value.insert("increment", 3u32).unwrap_or_revert();

    // Call Counter to get the current value.
    // let current_counter_value: i32 =
    //     runtime::call_contract(contract_hash, COUNTER_GET, RuntimeArgs::new());

    let args = runtime_args! {
        "increment" => 3u64,
    };
    // Call Counter to increment the value.
    let _: () = runtime::call_contract(contract_hash, COUNTER_INC, args);

    // Call Counter to get the new value.
    // let new_counter_value: i32 =
    //     runtime::call_contract(contract_hash, COUNTER_GET, RuntimeArgs::new());

    // Expect counter to increment by one.
    // if new_counter_value - current_counter_value != 3i32 {
    //     runtime::revert(ApiError::User(67));
    // }
}
