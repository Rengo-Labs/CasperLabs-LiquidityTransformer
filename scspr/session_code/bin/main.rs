#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256, U512
};

const SET_LIQUIDITY_TRANSFOMER: &str = "set_liquidity_transfomer";
const RESERVE_WISE: &str = "reserve_wise";
const FORWARD_LIQUIDITY: &str = "forward_liquidity";
const FUND_CONTRACT: &str = "fund_contract";

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");
    match entrypoint.as_str() {
        SET_LIQUIDITY_TRANSFOMER => {
            let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                SET_LIQUIDITY_TRANSFOMER,
                runtime_args! {
                    "immutable_transformer" => immutable_transformer,
                    "transformer_purse" => system::create_purse(),
                },
            );
        }
        RESERVE_WISE => {
            let investment_mode: u8 = runtime::get_named_arg("investment_mode");
            let msg_value: U256 = runtime::get_named_arg("msg_value");
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            let amount: U512 = runtime::get_named_arg("amount");
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None).unwrap_or_revert();
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                RESERVE_WISE,
                runtime_args! {
                    "investment_mode" => investment_mode,
                    "msg_value" => msg_value,
                    "caller_purse" => purse
                },
            );
        }
        FORWARD_LIQUIDITY => {
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            let amount: U512 = runtime::get_named_arg("amount");
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None).unwrap_or_revert();
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FORWARD_LIQUIDITY,
                runtime_args! {
                    "purse" => purse
                },
            );
        }
        FUND_CONTRACT => {
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            let amount: U512 = runtime::get_named_arg("amount");
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None).unwrap_or_revert();
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FUND_CONTRACT,
                runtime_args! {
                    "purse" => purse,
                    "amount" => amount
                },
            );
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
