#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};

pub const AMOUNT_RUNTIME_ARG: &str = "amount";
pub const PURSE_RUNTIME_ARG: &str = "purse";

pub const DESTINATION_DEPOSIT_SCSPR: &str = "deposit";
pub const MSG_VALUE_RUNTIME_ARG: &str = "msg_value";
pub const SUCCESOR_PURSE_RUNTIME_ARG: &str = "succesor_purse";

pub const DESTINATION_DEPOSIT: &str = "deposit_no_return";

pub const DESTINATION_SET_LIQUIDITY_TRANSFOMER: &str = "set_liquidity_transfomer";
pub const IMMUTABLE_TRANSFORMER_RUNTIME_ARG: &str = "immutable_transformer";
pub const TRANSFORMER_PURSE_RUNTIME_ARG: &str = "transformer_purse";

pub const DESTINATION_FORM_LIQUIDITY: &str = "form_liquidity";
pub const PAIR_RUNTIME_ARG: &str = "pair";

pub const DESTINATION_FORWARD_LIQUIDITY: &str = "forward_liquidity";

pub const DESTINATION_FUND_CONTRACT: &str = "fund_contract";

#[repr(u32)]
pub enum Error {
    Abort = 0,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
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
            runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_SET_LIQUIDITY_TRANSFOMER,
                runtime_args! {
                    IMMUTABLE_TRANSFORMER_RUNTIME_ARG => immutable_transformer,
                    TRANSFORMER_PURSE_RUNTIME_ARG => secondary_purse
                },
            )
        }
        DESTINATION_FORWARD_LIQUIDITY => runtime::call_versioned_contract(
            ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
            None,
            DESTINATION_FORWARD_LIQUIDITY,
            runtime_args! {
                PURSE_RUNTIME_ARG => secondary_purse,
            },
        ),
        DESTINATION_FORM_LIQUIDITY => {
            let pair: Key = runtime::get_named_arg(PAIR_RUNTIME_ARG);
            let ret: U256 = runtime::call_versioned_contract(
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
        _ => runtime::revert(ApiError::MissingKey),
    };
}
