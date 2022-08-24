use core::str::FromStr;

use crate::{data, errors::Error, event::SCSPREvent};
use alloc::vec::Vec;
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use contract_utils::{ContractContext, ContractStorage};
use synthetic_token_crate::{
    data::{self as synthetic_token_data, get_uniswap_router, get_wcspr, set_master_address},
    synthetic_helper_crate::data::{get_contract_purse, set_contract_purse, LIMIT_AMOUNT},
    SYNTHETICTOKEN,
};
use num_traits::cast::AsPrimitive;

use crate::alloc::{collections::BTreeMap, string::ToString};

pub trait SCSPR<Storage: ContractStorage>:
    ContractContext<Storage> + SYNTHETICTOKEN<Storage>
{
    fn init(
        &mut self,
        uniswap_factory: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        purse: URef
    ) {
        data::set_uniswap_factory(uniswap_factory);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        set_contract_purse(purse);
        data::set_owner(self.get_caller());
        set_master_address(self.get_caller());
    }

    fn set_master(&self, master_address: Key) {
        if self.get_caller() != data::get_owner() {
            runtime::revert(ApiError::from(Error::NotOwner));
        }
        synthetic_token_data::set_master_address(master_address);
    }

    fn set_wise(&self, wise: Key) {
        self.only_master();
        data::set_wise_contract(wise);
    }

    fn only_master(&self) {
        let master: Key = synthetic_token_data::get_master_address();
        if (self.get_caller() != master) && (master != data::zero_address()) {
            runtime::revert(ApiError::from(Error::InvalidAddress));
        }
    }

    fn only_transformer(&self) {
        if self.get_caller()
            != runtime::call_versioned_contract(
                data::get_wise_contract()
                    .into_hash()
                    .unwrap_or_revert()
                    .into(),
                None,
                "get_liquidity_transformer",
                runtime_args! {},
            )
        {
            runtime::revert(ApiError::from(Error::InvalidCallDetected));
        }
    }

    fn receive(&mut self, msg_value: U256, succesor_purse: URef) {
        let is_allow_deposit: bool = synthetic_token_data::get_allow_deposit();
        if !is_allow_deposit {
            runtime::revert(ApiError::from(Error::DepositDisabled));
        }
        let is_bypass_enabled: bool = synthetic_token_data::get_bypass_enabled();
        if is_bypass_enabled {
            self.deposit(msg_value, succesor_purse);
        }
    }

    fn deposit(&mut self, msg_value: U256, succesor_purse: URef) {
        let is_allow_deposit: bool = synthetic_token_data::get_allow_deposit();
        if !is_allow_deposit {
            runtime::revert(ApiError::from(Error::InvalidDeposit));
        }
        
        // Payable
        let amount: U512 = U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(msg_value));
        system::transfer_from_purse_to_purse(succesor_purse, get_contract_purse(), amount, None).unwrap_or_revert();
        
        let deposit_amount: U256 = msg_value;
        self._clean_up(deposit_amount);
        self._fees_decision();
        self._arbitrage_decision();
        self._settle_cspr(deposit_amount, succesor_purse);
        self._update_evaluation();
        self.scspr_emit(&SCSPREvent::DepositedLiquidity {
            deposit_amount,
            transformer_address: self.get_caller(),
        });
    }

    fn withdraw(&mut self, token_amount: U256, succesor_purse: URef) {
        self._clean_up(0.into());
        self._fees_decision();
        self._arbitrage_decision();
        self._settle_cspr(token_amount, succesor_purse);
        self._update_evaluation();
        self.scspr_emit(&SCSPREvent::Withdrawal {
            from_address: self.get_caller(),
            token_amount,
        });
    }

    fn _settle_cspr(&mut self, amount_withdraw: U256, succesor_purse: URef) {
        let (amount_wcspr, amount_scspr): (U256, U256) = self._remove_liquidity(amount_withdraw);

        self._unwrap(amount_wcspr);

        let contract_purse = get_contract_purse();
        let _ = system::transfer_from_purse_to_purse(
            contract_purse,
            succesor_purse,
            U512::from(amount_wcspr.as_usize()),
            None,
        );

        self.burn(self.get_caller(), amount_withdraw);
        self.burn(data::get_hash(), amount_scspr);
    }

    fn _settle_scspr(&mut self, amount_withdraw: U256) {
        self.mint(self.get_caller(), amount_withdraw);
        self.mint(data::get_hash(), LIMIT_AMOUNT);
        let () = runtime::call_versioned_contract(
            get_wcspr().into_hash().unwrap_or_revert().into(),
            None,
            "deposit",
            runtime_args! {
                "msg_value" => amount_withdraw
            },
        );
        self._add_liquidity(amount_withdraw, LIMIT_AMOUNT);
        self._self_burn();
    }

    fn liquidity_deposit(&mut self, purse: URef, msg_value: U256) {
        self.only_transformer();
        let is_allow_deposit: bool = synthetic_token_data::get_allow_deposit();
        if is_allow_deposit {
            runtime::revert(ApiError::from(Error::InvalidDeposit));
        }

        // Payable
        let amount: U512 = U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(msg_value));
        system::transfer_from_purse_to_purse(purse, get_contract_purse(), amount, None).unwrap_or_revert();
        
        self.mint(self.get_caller(), msg_value);
        let amount: U512 = U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(msg_value));
        system::transfer_from_purse_to_purse(purse, get_contract_purse(), amount, None).unwrap_or_revert();
        self.scspr_emit(&SCSPREvent::DepositedLiquidity {
            deposit_amount: msg_value,
            transformer_address: self.get_caller(),
        });
    }

    fn form_liquidity(&mut self, _pair: Option<Key>, purse: URef) -> U256 {
        self.only_transformer();
        let is_allow_deposit: bool = synthetic_token_data::get_allow_deposit();
        if is_allow_deposit {
            runtime::revert(ApiError::from(Error::InvalidState));
        }
        synthetic_token_data::set_allow_deposit(true);
        let cover_amount_: U512 = self._get_balance_half();
        let cover_amount: U256 = U256::from_str(cover_amount_.to_string().as_str()).unwrap();
        self.mint(Key::from(data::get_contract_package_hash()), cover_amount);
        self._approve(
            Key::from(data::get_contract_package_hash()),
            get_uniswap_router(),
            cover_amount,
        );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            get_wcspr().into_hash().unwrap_or_revert().into(),
            None,
            "deposit",
            runtime_args! {
                "purse" => purse,
                "amount" => cover_amount_ * cover_amount_
            },
        );
        if ret.is_err() {
            runtime::revert(ApiError::from(Error::AmountToTransferIsZero));
        }
        let () = runtime::call_versioned_contract(
            get_wcspr().into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => get_uniswap_router(),
                "amount" => cover_amount
            },
        );
        let zero: U256 = 0.into();
        let time: u64 = runtime::get_blocktime().into();
        let (amount_token_a, amount_token_b, liquidity): (U256, U256, U256) =
            runtime::call_versioned_contract(
                get_uniswap_router().into_hash().unwrap_or_revert().into(),
                None,
                "add_liquidity",
                runtime_args! {
                    "token_a" => get_wcspr(),
                    "token_b" => Key::from(data::get_contract_package_hash()),
                    "amount_a_desired" => cover_amount,
                    "amount_b_desired" => cover_amount,
                    "amount_a_min" => zero,
                    "amount_b_min" => zero,
                    "to" => Key::from(data::get_contract_package_hash()),
                    "deadline" => U256::from(time + 7200),
                    "pair" => _pair,
                },
            );
        self.scspr_emit(&SCSPREvent::FormedLiquidity {
            cover_amount,
            amount_token_a,
            amount_token_b,
            liquidity,
        });
        let remaining_balance: U512 =
            system::get_purse_balance(get_contract_purse()).unwrap_or_revert();
        self._profit(remaining_balance);
        self._update_evaluation();
        cover_amount
    }

    fn renounce_ownership(&mut self) {
        self.only_master();
        let zero_addr: Key = Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();

        synthetic_token_data::set_master_address(zero_addr);
    }

    fn forward_ownership(&mut self, new_master: Key) {
        self.only_master();

        synthetic_token_data::set_master_address(new_master);
    }

    fn add_lp_tokens(&mut self, purse: URef, msg_value: U256, token_amount: U256) {
        self.only_master();
        self.deposit(msg_value, purse);

        // Payable
        let amount: U512 = U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(msg_value));
        system::transfer_from_purse_to_purse(purse, get_contract_purse(), amount, None).unwrap_or_revert();

        let ret: Result<(), u32> = runtime::call_versioned_contract(
            get_uniswap_router().into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => self.get_caller(),
                "recipient" => Key::from(data::get_contract_package_hash()),
                "amount" => token_amount
            },
        );
        ret.unwrap_or_revert();
        self._update_evaluation();
    }

    fn define_token(&mut self, wise_token: Key) -> Key {
        self.only_master();
        let is_token_defined: bool = synthetic_token_data::get_token_defined();
        if is_token_defined {
            runtime::revert(ApiError::from(Error::AlreadyDefined));
        }

        let synthetic_cspr: Key = runtime::call_versioned_contract(
            wise_token.into_hash().unwrap_or_revert().into(),
            None,
            "get_synthetic_token_address",
            runtime_args! {},
        );

        if synthetic_cspr != Key::from(data::get_contract_package_hash()) {
            runtime::revert(ApiError::from(Error::InvalidWiseContractAddress));
        }

        synthetic_token_data::set_token_defined(true);

        synthetic_cspr
    }

    fn define_helper(&mut self, transfer_helper: Key) -> Key {
        self.only_master();
        let is_helper_defined: bool = synthetic_token_data::get_helper_defined();
        if is_helper_defined {
            runtime::revert(ApiError::from(Error::AlreadyDefined));
        }
        let transfer_invoker: Key = runtime::call_versioned_contract(
            transfer_helper.into_hash().unwrap_or_revert().into(),
            None,
            "get_transfer_invoker_address",
            runtime_args! {},
        );
        if transfer_invoker != Key::from(data::get_contract_package_hash()) {
            runtime::revert(ApiError::from(Error::InvalidTransferHelperAddress));
        }
        synthetic_token_data::set_helper_defined(true);
        transfer_invoker
    }

    fn create_pair(&mut self, pair: Key) {
        self.only_master();
        let () = runtime::call_versioned_contract(
            data::get_uniswap_factory()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "create_pair",
            runtime_args! {
                "token_a" => get_wcspr(),
                "token_b" => data::get_hash(),
                "pair_hash" => pair
            },
        );
        synthetic_token_data::set_uniswap_pair(pair);
    }

    fn scspr_emit(&mut self, erc20_event: &SCSPREvent) {
        let mut events = Vec::new();
        let tmp = data::get_contract_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split('-').collect();
        let package_hash = tmp[1].to_string();
        match erc20_event {
            SCSPREvent::DepositedLiquidity {
                deposit_amount,
                transformer_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", erc20_event.type_name());
                event.insert("deposit_amount", deposit_amount.to_string());
                event.insert("transformer_address", transformer_address.to_string());
                events.push(event);
            }
            SCSPREvent::Withdrawal {
                from_address,
                token_amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", erc20_event.type_name());
                event.insert("from_address", from_address.to_string());
                event.insert("token_amount", token_amount.to_string());
                events.push(event);
            }
            SCSPREvent::FormedLiquidity {
                cover_amount,
                amount_token_a,
                amount_token_b,
                liquidity,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", erc20_event.type_name());
                event.insert("cover_amount", cover_amount.to_string());
                event.insert("amount_token_a", amount_token_a.to_string());
                event.insert("amount_token_b", amount_token_b.to_string());
                event.insert("liquidity", liquidity.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
