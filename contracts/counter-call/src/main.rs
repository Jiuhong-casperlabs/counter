#![no_main]

extern crate alloc;

use casper_types::{
    bytesrepr::ToBytes, runtime_args, runtime_args::RuntimeArgs, ApiError, ContractHash, Key, U256,
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

    let mut vec = Vec::new();
    let value1 = U256::from_dec_str("1").unwrap();
    let value2 = U256::from_dec_str("2").unwrap();
    let value3 = U256::from_dec_str("3").unwrap();
    vec.push(value1);
    vec.push(value2);
    vec.push(value3);

    let _ = test.insert("hello", vec);

    // Call Counter to increment the value.
    let _: () = runtime::call_contract(contract_hash, COUNTER_INC, test);
}
