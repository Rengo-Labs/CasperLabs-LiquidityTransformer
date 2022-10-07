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
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256, U512,
};
use num_traits::AsPrimitive;

const RESERVE_WISE: &str = "reserve_wise";
const SET_LIQUIDITY_TRANSFOMER: &str = "set_liquidity_transfomer";
const DEPOSIT: &str = "deposit";
const WCSPR_DEPOSIT: &str = "wcspr_deposit";
const WITHDRAW: &str = "withdraw";
const TRANSFER: &str = "transfer";
const BALANCE_OF: &str = "balance_of";
const ADD_LP_TOKENS: &str = "add_lp_tokens";
const GET_WRAPPED_BALANCE: &str = "get_wrapped_balance";
const GET_SYNTHETIC_BALANCE: &str = "get_synthetic_balance";

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
        RESERVE_WISE => {
            let investment_mode: u8 = runtime::get_named_arg("investment_mode");
            let amount: U512 = runtime::get_named_arg("amount");
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None)
                .unwrap_or_revert();
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                RESERVE_WISE,
                runtime_args! {
                    "investment_mode" => investment_mode,
                    "msg_value" => <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount),
                    "caller_purse" => purse
                },
            );
        }
        SET_LIQUIDITY_TRANSFOMER => {
            let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
            let transformer_purse: URef = runtime::call_versioned_contract(
                immutable_transformer.into_hash().unwrap_or_revert().into(),
                None,
                "contract_read_only_purse",
                runtime_args! {},
            );
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                SET_LIQUIDITY_TRANSFOMER,
                runtime_args! {
                    "immutable_transformer" => immutable_transformer,
                    "transformer_purse" => transformer_purse
                },
            );
        }
        DEPOSIT => {
            let amount: U512 = runtime::get_named_arg("amount");
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None)
                .unwrap_or_revert();
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DEPOSIT,
                runtime_args! {
                    "purse" => purse,
                    "amount" => amount
                },
            );
        }
        WCSPR_DEPOSIT => {
            let amount: U512 = runtime::get_named_arg("amount");
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None)
                .unwrap_or_revert();
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DEPOSIT,
                runtime_args! {
                    "purse" => purse,
                    "amount" => amount
                },
            );
            ret.unwrap_or_revert();
        }
        WITHDRAW => {
            let amount: U512 = runtime::get_named_arg("amount");
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None)
                .unwrap_or_revert();
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                WITHDRAW,
                runtime_args! {
                    "purse" => purse,
                    "amount" => amount
                },
            );
        }
        TRANSFER => {
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TRANSFER,
                runtime_args! {
                    "recipient" => recipient,
                    "amount" => amount
                },
            );
            ret.unwrap_or_revert();
        }
        BALANCE_OF => {
            let owner: Key = runtime::get_named_arg("owner");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF,
                runtime_args! {
                    "owner" => owner
                },
            );
            store(BALANCE_OF, ret);
        }
        ADD_LP_TOKENS => {
            let amount: U512 = runtime::get_named_arg("amount");
            let caller_purse = account::get_main_purse();
            let purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None)
                .unwrap_or_revert();
            let token_amount: U256 = runtime::get_named_arg("token_amount");
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                ADD_LP_TOKENS,
                runtime_args! {
                    "purse" => purse,
                    "amount" => <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount),
                    "token_amount" => token_amount,
                },
            );
        }
        GET_SYNTHETIC_BALANCE => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_SYNTHETIC_BALANCE,
                runtime_args! {},
            );
            store(GET_SYNTHETIC_BALANCE, ret);
        }
        GET_WRAPPED_BALANCE => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_WRAPPED_BALANCE,
                runtime_args! {},
            );
            store(GET_WRAPPED_BALANCE, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
