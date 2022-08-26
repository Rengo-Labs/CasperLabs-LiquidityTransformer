use alloc::{vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, URef, U256, U512};
use casperlabs_contract_utils::{ContractContext, ContractStorage};

use crate::data;

#[repr(u16)]
pub enum Error {
    BalanceNotFound = 0,
    BalanceMismatch,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait SYNTHETICHELPER<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, contract_purse: URef) {
        data::set_contract_purse(contract_purse);
    }

    fn _prepare_path(&mut self, token_from: Key, token_to: Key) -> Vec<Key> {
        vec![token_from, token_to]
    }

    fn _get_double_root(&mut self, amount: U256) -> U256 {
        amount.integer_sqrt() * 2
    }

    fn _get_balance_half(&mut self) -> U512 {
        system::get_purse_balance(data::get_contract_purse())
            .unwrap_or_revert()
            .checked_div(2.into())
            .unwrap_or_revert()
    }

    fn _get_balance_diff(&mut self, amount: U256) -> U512 {
        if system::get_purse_balance(data::get_contract_purse()).unwrap_or_revert()
            > U512::from(amount.as_usize())
        {
            return system::get_purse_balance(data::get_contract_purse()).unwrap_or_revert()
                - U512::from(amount.as_usize());
        }
        0.into()
    }

    fn _get_balance_of(&mut self, token: Key, owner: Key) -> U256 {
        // Generic Token
        runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => owner
            },
        )
    }

    fn fund_contract(&mut self, purse: URef, amount: U512) {
        system::transfer_from_purse_to_purse(purse, data::get_contract_purse(), amount, None)
            .unwrap_or_revert();
    }
}
