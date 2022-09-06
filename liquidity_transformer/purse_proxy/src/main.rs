#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, ContractPackageHash, Key, RuntimeArgs,
    URef, U256, U512,
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

pub const DESTINATION_DEPOSIT_SCSPR: &str = "deposit";
pub const DESTINATION_DEPOSIT: &str = "deposit_no_return";
pub const DESTINATION_SET_LIQUIDITY_TRANSFOMER: &str = "set_liquidity_transfomer";
pub const DESTINATION_FORM_LIQUIDITY: &str = "form_liquidity";
pub const DESTINATION_FUND_CONTRACT: &str = "fund_contract";
pub const DESTINATION_RESERVE_WISE: &str = "reserve_wise";
pub const DESTINATION_REQUEST_REFUND: &str = "request_refund";

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

#[no_mangle]
pub extern "C" fn call() {
    let destination_package_hash: Key = runtime::get_named_arg("destination_package_hash");
    let destination_entrypoint: String = runtime::get_named_arg("destination_entrypoint");
    let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);

    let main_purse: URef = account::get_main_purse();
    let secondary_purse: URef = system::create_purse();

    system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
        .unwrap_or_revert();

    match destination_entrypoint.as_str() {
        DESTINATION_DEPOSIT => runtime::call_versioned_contract(
            ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
            None,
            DESTINATION_DEPOSIT,
            runtime_args! {
                AMOUNT_RUNTIME_ARG => amount,
                PURSE_RUNTIME_ARG => secondary_purse
            },
        ),
        DESTINATION_FUND_CONTRACT => runtime::call_versioned_contract(
            ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
            None,
            DESTINATION_FUND_CONTRACT,
            runtime_args! {
                AMOUNT_RUNTIME_ARG => amount,
                PURSE_RUNTIME_ARG => secondary_purse
            },
        ),
        DESTINATION_SET_LIQUIDITY_TRANSFOMER => {
            let immutable_transformer: Key =
                runtime::get_named_arg(IMMUTABLE_TRANSFORMER_RUNTIME_ARG);
            let transformer_purse: URef = runtime::call_versioned_contract(
                immutable_transformer.into_hash().unwrap_or_revert().into(),
                None,
                "contract_read_only_purse",
                runtime_args! {},
            );
            runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_SET_LIQUIDITY_TRANSFOMER,
                runtime_args! {
                    IMMUTABLE_TRANSFORMER_RUNTIME_ARG => immutable_transformer,
                    TRANSFORMER_PURSE_RUNTIME_ARG => transformer_purse
                },
            )
        }
        DESTINATION_FORM_LIQUIDITY => {
            let pair: Key = runtime::get_named_arg(PAIR_RUNTIME_ARG);
            let _ret: U256 = runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_FORM_LIQUIDITY,
                runtime_args! {
                    PURSE_RUNTIME_ARG => secondary_purse,
                    PAIR_RUNTIME_ARG => pair
                },
            );
        }
        DESTINATION_DEPOSIT_SCSPR => runtime::call_versioned_contract(
            ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
            None,
            DESTINATION_DEPOSIT_SCSPR,
            runtime_args! {
                MSG_VALUE_RUNTIME_ARG => amount,
                SUCCESOR_PURSE_RUNTIME_ARG => secondary_purse
            },
        ),
        DESTINATION_RESERVE_WISE => {
            let investment_mode: u8 = runtime::get_named_arg(INVESTMENT_MODE_RUNTIME_ARG);
            let () = runtime::call_versioned_contract(
                destination_package_hash
                    .into_hash()
                    .unwrap_or_revert()
                    .into(),
                None,
                DESTINATION_RESERVE_WISE,
                runtime_args! {
                    INVESTMENT_MODE_RUNTIME_ARG => investment_mode,
                    MSG_VALUE_RUNTIME_ARG => <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(amount),
                    CALLER_PURSE_RUNTIME_ARG => secondary_purse
                },
            );
        }
        DESTINATION_REQUEST_REFUND => {
            let ret: (U256, U256) = runtime::call_versioned_contract(
                destination_package_hash
                    .into_hash()
                    .unwrap_or_revert()
                    .into(),
                None,
                DESTINATION_REQUEST_REFUND,
                runtime_args! {
                    CALLER_PURSE_RUNTIME_ARG => secondary_purse
                },
            );
            store("result", ret);
        }
        _ => runtime::revert(ApiError::MissingKey),
    };
}
