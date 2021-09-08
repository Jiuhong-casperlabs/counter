#![no_main]
#![no_std]

extern crate alloc;
use core::convert::TryInto;
use core::str::FromStr;

use alloc::string::ToString;
use alloc::vec;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::contracts::NamedKeys;
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, Key, Parameter, URef,
};
use casper_types::{AccessRights, CLTyped, U128, U256, U512};

const COUNT_KEY: &str = "count";
const COUNTER_INC: &str = "counter_inc";

const COUNTER_KEY: &str = "counter";

pub const DICTIONARY_NAME: &str = "local";
pub const DICTIONARY_REF: &str = "new_dictionary";
pub const DEFAULT_DICTIONARY_NAME: &str = "sun";
pub const DEFAULT_DICTIONARY_VALUE: &str = "jiuhong";
pub const GET_RESULT: &str = "getdicresult";
const CONTRACT_PACKAGE_HASH_NAME: &str = "package_hash_name";

#[no_mangle]
pub extern "C" fn counter_inc() {
    let increment: u64 = runtime::get_named_arg("increment");
    // let increment = i32::from_str(&increment_str).unwrap();
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, increment);
}

#[no_mangle]
#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = storage::new_uref(0); //initialize counter

    // Create initial named keys of the contract.
    let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from(COUNT_KEY);
    counter_named_keys.insert(key_name, counter_local_key.into());

    // Create entry point
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        COUNTER_INC,
        vec![Parameter::new("increment", CLType::U64)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None);
    runtime::put_key(COUNTER_KEY, stored_contract_hash.into());
}
