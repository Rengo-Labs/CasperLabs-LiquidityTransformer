#![no_std]
#![no_main]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256, U512,
};
use num_traits::cast::AsPrimitive;

pub const AMOUNT_RUNTIME_ARG: &str = "amount";
pub const PURSE_RUNTIME_ARG: &str = "purse";
pub const SUCCESOR_PURSE_RUNTIME_ARG: &str = "succesor_purse";
pub const MSG_VALUE_RUNTIME_ARG: &str = "msg_value";
pub const IMMUTABLE_TRANSFORMER_RUNTIME_ARG: &str = "immutable_transformer";
pub const TRANSFORMER_PURSE_RUNTIME_ARG: &str = "transformer_purse";
pub const PAIR_RUNTIME_ARG: &str = "pair";
pub const CALLER_PURSE_RUNTIME_ARG: &str = "caller_purse";
pub const INVESTMENT_MODE_RUNTIME_ARG: &str = "investment_mode";
pub const TOKEN_ADDRESS_RUNTIME_ARG: &str = "token_address";
pub const TOKEN_AMOUNT_RUNTIME_ARG: &str = "token_amount";
pub const INVESTOR_ADDRESS_RUNTIME_ARG: &str = "investor_address";

pub const DEPOSIT: &str = "deposit";
pub const SET_LIQUIDITY_TRANSFOMER: &str = "set_liquidity_transfomer";
pub const FORM_LIQUIDITY: &str = "form_liquidity";
pub const FUND_CONTRACT: &str = "fund_contract";
pub const RESERVE_WISE: &str = "reserve_wise";
pub const RESERVE_WISE_WITH_TOKEN: &str = "reserve_wise_with_token";
pub const REQUEST_REFUND: &str = "request_refund";
pub const CURRENT_STAKEABLE_DAY: &str = "current_stakeable_day";
pub const PAYOUT_INVESTOR_ADDRESS: &str = "payout_investor_address";
pub const PREPARE_PATH: &str = "prepare_path";

#[repr(u32)]
pub enum Error {
    Abort = 0,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

fn temp_purse(amount: U512) -> URef {
    let main_purse: URef = account::get_main_purse();
    let secondary_purse: URef = system::create_purse();
    system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
        .unwrap_or_revert();
    secondary_purse
}

#[no_mangle]
pub extern "C" fn call() {
    let package_hash: Key = runtime::get_named_arg("package_hash");
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    match entrypoint.as_str() {
        DEPOSIT => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse = temp_purse(amount);
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DEPOSIT,
                runtime_args! {
                    AMOUNT_RUNTIME_ARG => amount,
                    PURSE_RUNTIME_ARG => secondary_purse
                },
            );
            store(DEPOSIT, ret);
        }
        FUND_CONTRACT => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse = temp_purse(amount);
            runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FUND_CONTRACT,
                runtime_args! {
                    AMOUNT_RUNTIME_ARG => amount,
                    PURSE_RUNTIME_ARG => secondary_purse
                },
            )
        }
        SET_LIQUIDITY_TRANSFOMER => {
            let immutable_transformer: Key =
                runtime::get_named_arg(IMMUTABLE_TRANSFORMER_RUNTIME_ARG);
            let transformer_purse: URef = runtime::call_versioned_contract(
                immutable_transformer.into_hash().unwrap_or_revert().into(),
                None,
                "contract_read_only_purse",
                runtime_args! {},
            );
            runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                SET_LIQUIDITY_TRANSFOMER,
                runtime_args! {
                    IMMUTABLE_TRANSFORMER_RUNTIME_ARG => immutable_transformer,
                    TRANSFORMER_PURSE_RUNTIME_ARG => transformer_purse
                },
            )
        }
        FORM_LIQUIDITY => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse = temp_purse(amount);
            let pair: Key = runtime::get_named_arg(PAIR_RUNTIME_ARG);
            let _ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FORM_LIQUIDITY,
                runtime_args! {
                    PURSE_RUNTIME_ARG => secondary_purse,
                    PAIR_RUNTIME_ARG => pair
                },
            );
        }
        RESERVE_WISE => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse = temp_purse(amount);
            let investment_mode: u8 = runtime::get_named_arg(INVESTMENT_MODE_RUNTIME_ARG);
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                RESERVE_WISE,
                runtime_args! {
                    INVESTMENT_MODE_RUNTIME_ARG => investment_mode,
                    MSG_VALUE_RUNTIME_ARG => <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount),
                    CALLER_PURSE_RUNTIME_ARG => secondary_purse
                },
            );
        }
        RESERVE_WISE_WITH_TOKEN => {
            let token_address: Key = runtime::get_named_arg("token_address");
            let token_amount: U256 = runtime::get_named_arg("token_amount");
            let investment_mode: u8 = runtime::get_named_arg("investment_mode");
            let () = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                RESERVE_WISE_WITH_TOKEN,
                runtime_args! {
                    TOKEN_ADDRESS_RUNTIME_ARG => token_address,
                    TOKEN_AMOUNT_RUNTIME_ARG => token_amount,
                    INVESTMENT_MODE_RUNTIME_ARG => investment_mode,
                    CALLER_PURSE_RUNTIME_ARG => account::get_main_purse()
                },
            );
        }
        REQUEST_REFUND => {
            let ret: (U256, U256) = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                REQUEST_REFUND,
                runtime_args! {
                    CALLER_PURSE_RUNTIME_ARG => account::get_main_purse()
                },
            );
            store(REQUEST_REFUND, ret);
        }
        CURRENT_STAKEABLE_DAY => {
            let ret: u64 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CURRENT_STAKEABLE_DAY,
                runtime_args! {},
            );
            store(CURRENT_STAKEABLE_DAY, ret);
        }
        PAYOUT_INVESTOR_ADDRESS => {
            let investor_address: Key = runtime::get_named_arg(INVESTOR_ADDRESS_RUNTIME_ARG);
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                PAYOUT_INVESTOR_ADDRESS,
                runtime_args! {
                    INVESTOR_ADDRESS_RUNTIME_ARG => investor_address
                },
            );
            store(PAYOUT_INVESTOR_ADDRESS, ret);
        }
        PREPARE_PATH => {
            let token_address: Key = runtime::get_named_arg(TOKEN_ADDRESS_RUNTIME_ARG);
            let ret: Vec<Key> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                PREPARE_PATH,
                runtime_args! {
                    TOKEN_ADDRESS_RUNTIME_ARG => token_address
                },
            );
            store(PREPARE_PATH, ret);
        }
        _ => runtime::revert(ApiError::MissingKey),
    };
}
