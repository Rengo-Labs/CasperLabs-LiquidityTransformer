use core::convert::TryInto;

use alloc::string::{String, ToString};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn self_hash_key() -> String {
    "self_hash".to_string()
}

pub fn self_package_key() -> String {
    "package_hash".to_string()
}

pub fn scspr_key() -> String {
    "scspr".to_string()
}

pub fn form_liquidity_key() -> String {
    "form_liquidity_result".to_string()
}

pub fn transfer_helper_key() -> String {
    "transfer_helper_result".to_string()
}

pub fn transfer_approve_key() -> String {
    "transfer_approve_result".to_string()
}

pub fn token0_key() -> String {
    "token0_result".to_string()
}

pub fn token1_key() -> String {
    "token1_result".to_string()
}

pub fn result_key() -> String {
    "result".to_string()
}
