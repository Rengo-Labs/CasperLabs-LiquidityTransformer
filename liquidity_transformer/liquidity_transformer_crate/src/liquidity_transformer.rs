use alloc::vec;
use alloc::vec::Vec;
use core::str::FromStr;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, URef, U256, U512};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use num_traits::cast::AsPrimitive;
use renvm_sig::keccak256;

use crate::data::{self, *};

#[repr(u16)]
pub enum Error {
    ReserveWiseMaxSupplyReached = 0,
    ReserveWrongInvestmentDay,
    ReserveWiseMinInvest,
    ReserverWiseWrongMode,
    NotKeeper,
    Swapped,
    RefundNotPossible,
    InvestmentBelowMinimum,
    OngoingInvestmentPhase,
    ForwardLiquidityFirst,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub enum LiquidityTransformerEvent {
    WiseReservation {
        sender_address: Key,
        investment_amount: U256,
        token_amount: U256,
        current_stakeable_day: u64,
        investment_mode: u8,
    },
    UniswapSwapResult {
        amount_token_a: U256,
        amount_token_b: U256,
        liquidity: U256,
    },
    CashBackIssued {
        investor_address: Key,
        sender_value: U256,
        cash_back_amount: U256,
    },
    RefundIssued {
        investor_address: Key,
        refund_amount: U256,
    },
}

impl LiquidityTransformerEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidityTransformerEvent::WiseReservation {
                sender_address: _,
                investment_amount: _,
                token_amount: _,
                current_stakeable_day: _,
                investment_mode: _,
            } => "wiseReservation",
            LiquidityTransformerEvent::UniswapSwapResult {
                amount_token_a: _,
                amount_token_b: _,
                liquidity: _,
            } => "uniswapSwapResult",
            LiquidityTransformerEvent::CashBackIssued {
                investor_address: _,
                sender_value: _,
                cash_back_amount: _,
            } => "cashBackIssued",
            LiquidityTransformerEvent::RefundIssued {
                investor_address: _,
                refund_amount: _,
            } => "refundIssued",
        }
        .to_string()
    }
}

pub trait LIQUIDITYTRANSFORMER<Storage: ContractStorage>: ContractContext<Storage> {
    #[allow(clippy::too_many_arguments)]
    fn init(
        &self,
        wise_token: Key,
        scspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
        wcspr: Key,
        package_hash: Key,
        contract_hash: Key,
        purse: URef,
    ) {
        data::set_wise(wise_token);
        data::set_scspr(scspr);
        data::set_uniswap_pair(uniswap_pair);
        data::set_uniswap_router(uniswap_router);
        data::set_wcspr(wcspr);
        data::set_hash(contract_hash);
        data::set_package(package_hash);
        data::set_settings_keeper(self.get_caller());
        data::set_self_purse(purse);

        Globals::init();
        UniqueInvestors::init();
        PurchasedTokens::init();
        InvestorBalance::init();
    }

    // --- MODIFIERS --- //

    fn after_investment_days(&self) {
        if self.current_stakeable_day() <= data::INVESTMENT_DAYS as u64 {
            runtime::revert(ApiError::from(Error::OngoingInvestmentPhase));
        }
    }

    fn after_uniswap_transfer(&self) {
        let ret: bool = data::Globals::instance().get(UNISWAP_SWAPED);
        if !ret {
            runtime::revert(ApiError::from(Error::ForwardLiquidityFirst));
        }
    }

    fn below_maximum_invest(&self) {
        let ret: U256 = data::Globals::instance().get(TOTAL_TRANSFER_TOKENS);
        if ret >= U256::from(data::MAX_SUPPLY) {
            runtime::revert(ApiError::from(Error::ReserveWiseMaxSupplyReached));
        }
    }

    fn below_maximum_day(&self) {
        if self.current_stakeable_day() == 0
            || self.current_stakeable_day() > data::INVESTMENT_DAYS as u64
        {
            runtime::revert(ApiError::from(Error::ReserveWrongInvestmentDay));
        }
    }

    fn only_keeper(&self) {
        if self.get_caller() != data::settings_keeper() {
            runtime::revert(ApiError::from(Error::NotKeeper));
        }
    }

    // --- FUNCTIONS --- //

    fn set_settings(&self, wise_token: Key, uniswap_pair: Key, synthetic_cspr: Key) {
        self.only_keeper();
        data::set_wise(wise_token);
        data::set_scspr(synthetic_cspr);
        data::set_uniswap_pair(uniswap_pair);
    }

    fn renounce_keeper(&self) {
        self.only_keeper();
        data::set_settings_keeper(data::zero_address());
    }

    fn reserve_wise(&mut self, investment_mode: u8, msg_value: U256, caller_purse: URef) {
        self.below_maximum_day();
        self.below_maximum_invest();
        if msg_value < U256::from(data::TOKEN_COST) {
            runtime::revert(ApiError::from(Error::ReserveWiseMinInvest));
        }
        // Payable
        let amount: U512 =
            U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(msg_value));
        system::transfer_from_purse_to_purse(caller_purse, data::self_purse(), amount, None)
            .unwrap_or_revert();
        self._reserve_wise(self.get_caller(), msg_value, investment_mode, caller_purse);
    }

    fn reserve_wise_with_token(
        &mut self,
        token_address: Key,
        token_amount: U256,
        investment_mode: u8,
        caller_purse: URef,
    ) {
        self.below_maximum_day();
        self.below_maximum_invest();

        let args: RuntimeArgs = runtime_args! {
            "owner" => self.get_caller(),
            "recipient" => data::package(),
            "amount" => token_amount
        };
        let _: Result<(), u32> = runtime::call_versioned_contract(
            token_address.into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            args,
        );

        let args: RuntimeArgs = runtime_args! {
            "spender" => data::uniswap_router(),
            "amount" => token_amount
        };
        let () = runtime::call_versioned_contract(
            token_address.into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            args,
        );

        let _path: Vec<Key> = self.prepare_path(token_address);
        let path: Vec<String> = vec![
            _path[0].to_formatted_string(),
            _path[1].to_formatted_string(),
        ];

        let time: u64 = runtime::get_blocktime().into();
        let args: RuntimeArgs = runtime_args! {
            "amount_in" => token_amount,
            "amount_out_min" => U256::from(0),
            "path" => path,
            "to" => data::self_purse(),
            "deadline" => U256::from(time + 7_200_000)
        };
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            data::uniswap_router().into_hash().unwrap_or_revert().into(),
            None,
            "swap_exact_tokens_for_cspr",
            args,
        );

        if amounts[1] < U256::from(data::TOKEN_COST) {
            runtime::revert(ApiError::from(Error::InvestmentBelowMinimum));
        }

        self._reserve_wise(self.get_caller(), amounts[1], investment_mode, caller_purse);
    }

    fn _reserve_wise(
        &mut self,
        sender_address: Key,
        sender_value: U256,
        investment_mode: u8,
        caller_purse: URef,
    ) {
        let sender_address_hash =
            hex::encode(keccak256(sender_address.to_formatted_string().as_bytes()));

        if investment_mode >= 6 {
            runtime::revert(ApiError::from(Error::ReserverWiseWrongMode));
        }

        if InvestorBalance::instance().get(&sender_address_hash) == U256::from(0) {
            let ret: U256 = data::Globals::instance().get(INVESTOR_COUNT);
            UniqueInvestors::instance().set(&ret, sender_address);
            let ret: U256 = data::Globals::instance().get(INVESTOR_COUNT);
            data::Globals::instance().set(INVESTOR_COUNT, ret + 1);
        }

        let (sender_tokens, return_amount): (U256, U256) = self._get_token_amount(
            data::Globals::instance().get(TOTAL_CSPR_CONTRIBUTED),
            data::Globals::instance().get(TOTAL_TRANSFER_TOKENS),
            sender_value,
        );

        let ret: U256 = data::Globals::instance().get(TOTAL_CSPR_CONTRIBUTED);
        data::Globals::instance().set(TOTAL_CSPR_CONTRIBUTED, ret + sender_value);
        let ret: U256 = data::Globals::instance().get(TOTAL_TRANSFER_TOKENS);
        data::Globals::instance().set(TOTAL_TRANSFER_TOKENS, ret + sender_tokens);

        InvestorBalance::instance().set(
            &sender_address_hash,
            InvestorBalance::instance().get(&sender_address_hash) + sender_value,
        );
        PurchasedTokens::instance().set(
            &sender_address_hash,
            PurchasedTokens::instance().get(&sender_address_hash) + sender_tokens,
        );

        let ret: U256 = data::Globals::instance().get(CASH_BACK_TOTAL);
        if investment_mode == 0
            && ret < U256::from(data::REFUND_CAP)
            && return_amount < sender_value
        {
            let mut cash_back_amount: U256 = sender_value
                .checked_sub(return_amount)
                .unwrap_or_revert()
                .checked_div(100.into())
                .unwrap_or_revert();

            let mut cash_back: U256 = data::Globals::instance().get(CASH_BACK_TOTAL);
            cash_back = cash_back.checked_add(cash_back_amount).unwrap_or_revert();

            if cash_back >= U256::from(data::REFUND_CAP) {
                cash_back_amount = U256::from(REFUND_CAP)
                    .checked_sub(data::Globals::instance().get(CASH_BACK_TOTAL))
                    .unwrap_or_revert();
            }

            let mut ret: U256 = data::Globals::instance().get(CASH_BACK_TOTAL);
            ret = ret.checked_add(cash_back_amount).unwrap_or_revert();
            data::Globals::instance().set(CASH_BACK_TOTAL, ret);

            let _ = system::transfer_from_purse_to_purse(
                data::self_purse(),
                caller_purse,
                U512::from(
                    <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(cash_back_amount),
                ),
                None,
            );

            self.emit(&LiquidityTransformerEvent::CashBackIssued {
                investor_address: sender_address,
                sender_value,
                cash_back_amount,
            });
        }

        if return_amount > U256::from(0) {
            system::transfer_from_purse_to_purse(
                data::self_purse(),
                caller_purse,
                U512::from(
                    <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(return_amount),
                ),
                None,
            )
            .unwrap_or_revert();

            self.emit(&LiquidityTransformerEvent::RefundIssued {
                investor_address: self.get_caller(),
                refund_amount: return_amount,
            });
        }

        self.emit(&LiquidityTransformerEvent::WiseReservation {
            sender_address,
            investment_amount: sender_value,
            token_amount: sender_tokens,
            current_stakeable_day: self.current_stakeable_day(),
            investment_mode,
        });
    }

    fn _get_token_amount(
        &self,
        total_cspr_contributed: U256,
        total_transfer_tokens: U256,
        sender_value: U256,
    ) -> (U256, U256) {
        let mut token_amount: U256 = sender_value
            .checked_div(data::TOKEN_COST.into())
            .unwrap_or_revert()
            .checked_mul(U256::from(1_000_000_000_u128))
            .unwrap_or_revert();

        let new_supply: U256 = total_transfer_tokens
            .checked_add(token_amount)
            .unwrap_or_revert();

        let mut return_amount: U256 = 0.into();
        if new_supply > U256::from(data::MAX_SUPPLY) {
            token_amount = U256::from(
                data::MAX_SUPPLY
                    .checked_sub(total_transfer_tokens.as_u128())
                    .unwrap_or_revert(),
            );
            let available_value = MAX_INVEST
                .checked_sub(total_cspr_contributed.as_u128())
                .unwrap_or_revert();
            return_amount = sender_value
                .checked_sub(U256::from(available_value))
                .unwrap_or_revert();
        }

        (token_amount, return_amount)
    }

    fn forward_liquidity(&mut self) {
        self.after_investment_days();
        if data::Globals::instance().get(UNISWAP_SWAPED) {
            runtime::revert(ApiError::from(Error::Swapped));
        }
        let scspr_tokens_amount: U256 = data::Globals::instance().get(TOTAL_CSPR_CONTRIBUTED);
        let wise_tokens_amount: U256 = data::Globals::instance().get(TOTAL_TRANSFER_TOKENS);

        let total_cspr_contributed: U256 = data::Globals::instance().get(TOTAL_CSPR_CONTRIBUTED);
        let () = runtime::call_versioned_contract(
            data::scspr().into_hash().unwrap_or_revert().into(),
            None,
            "liquidity_deposit",
            runtime_args! {
                "purse" => data::self_purse(),
                "msg_value" => total_cspr_contributed
            },
        );

        let _: U256 = runtime::call_versioned_contract(
            data::scspr().into_hash().unwrap_or_revert().into(),
            None,
            "form_liquidity",
            runtime_args! {
              "pair" => data::uniswap_pair()
            },
        );

        let () = runtime::call_versioned_contract(
            data::scspr().into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => data::uniswap_router(),
                "amount" => scspr_tokens_amount
            },
        );

        let () = runtime::call_versioned_contract(
            data::wise().into_hash().unwrap_or_revert().into(),
            None,
            "mint_supply",
            runtime_args! {
                "investor_address" => data::package(),
                "amount" => wise_tokens_amount
            },
        );

        let () = runtime::call_versioned_contract(
            data::wise().into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => data::uniswap_router(),
                "amount" => wise_tokens_amount
            },
        );

        let () = runtime::call_versioned_contract(
            data::wise().into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => data::uniswap_router(),
                "amount" => wise_tokens_amount
            },
        );

        let time: u64 = runtime::get_blocktime().into();
        let (amount_token_a, amount_token_b, liquidity): (U256, U256, U256) =
            runtime::call_versioned_contract(
                data::uniswap_router().into_hash().unwrap_or_revert().into(),
                None,
                "add_liquidity",
                runtime_args! {
                    "token_a" => data::wise(),
                    "token_b" => data::scspr(),
                    "amount_a_desired" => wise_tokens_amount,
                    "amount_b_desired" => scspr_tokens_amount,
                    "amount_a_min" => U256::from(0),
                    "amount_b_min" => U256::from(0),
                    "to" => data::zero_address(),
                    "deadline" => U256::from(time + 7_200_000),
                    "pair" => Some(data::uniswap_pair())
                },
            );

        data::Globals::instance().set(UNISWAP_SWAPED, true);

        self.emit(&LiquidityTransformerEvent::UniswapSwapResult {
            amount_token_a,
            amount_token_b,
            liquidity,
        });
    }

    fn get_my_tokens(&self) {
        self.after_uniswap_transfer();
        self.payout_investor_address(self.get_caller());
    }

    fn payout_investor_address(&self, investor_address: Key) -> U256 {
        let investor_address_hash =
            hex::encode(keccak256(investor_address.to_formatted_string().as_bytes()));

        self.after_uniswap_transfer();
        let payout: U256 = PurchasedTokens::instance().get(&investor_address_hash);
        PurchasedTokens::instance().set(&investor_address_hash, 0.into());

        if payout > U256::from(0) {
            let () = runtime::call_versioned_contract(
                data::wise().into_hash().unwrap_or_revert().into(),
                None,
                "mint_supply",
                runtime_args! {
                    "investor_address" => investor_address,
                    "amount" => payout
                },
            );
        }
        payout
    }

    fn prepare_path(&self, token_address: Key) -> Vec<Key> {
        vec![token_address, data::wcspr()]
    }

    fn current_stakeable_day(&self) -> u64 {
        runtime::call_versioned_contract(
            data::wise().into_hash().unwrap_or_revert().into(),
            None,
            "current_stakeable_day",
            runtime_args! {},
        )
    }

    fn request_refund(&mut self, caller_purse: URef) -> (U256, U256) {
        let caller_address_hash = hex::encode(keccak256(
            self.get_caller().to_formatted_string().as_bytes(),
        ));

        let ret: bool = data::Globals::instance().get(UNISWAP_SWAPED);
        if ret
            || InvestorBalance::instance().get(&caller_address_hash) <= U256::from(0)
            || PurchasedTokens::instance().get(&caller_address_hash) <= U256::from(0)
            || self.current_stakeable_day() <= (data::INVESTMENT_DAYS + 10) as u64
        {
            runtime::revert(ApiError::from(Error::RefundNotPossible));
        }

        let amount: U256 = InvestorBalance::instance().get(&caller_address_hash);
        InvestorBalance::instance().set(&caller_address_hash, 0.into());

        let tokens: U256 = PurchasedTokens::instance().get(&caller_address_hash);
        PurchasedTokens::instance().set(&caller_address_hash, 0.into());

        let ret: U256 = data::Globals::instance().get(TOTAL_TRANSFER_TOKENS);
        data::Globals::instance().set(
            TOTAL_TRANSFER_TOKENS,
            ret.checked_sub(tokens).unwrap_or_revert(),
        );

        if amount > U256::from(0) {
            system::transfer_from_purse_to_purse(
                data::self_purse(),
                caller_purse,
                U512::from(<casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(amount)),
                None,
            )
            .unwrap_or_revert();
            self.emit(&LiquidityTransformerEvent::RefundIssued {
                investor_address: self.get_caller(),
                refund_amount: amount,
            });
        }

        (amount, tokens)
    }

    fn fund_contract(&mut self, purse: URef, amount: U512) {
        system::transfer_from_purse_to_purse(purse, data::self_purse(), amount, None)
            .unwrap_or_revert();
    }

    fn emit(&mut self, liquidity_transformer_event: &LiquidityTransformerEvent) {
        let mut events = Vec::new();
        let tmp = data::package().to_formatted_string();
        let tmp: Vec<&str> = tmp.split('-').collect();
        let package_hash = tmp[1].to_string();
        match liquidity_transformer_event {
            LiquidityTransformerEvent::WiseReservation {
                sender_address,
                investment_amount,
                token_amount,
                current_stakeable_day,
                investment_mode,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_transformer_event.type_name());
                event.insert("sender_address", sender_address.to_string());
                event.insert("investment_amount", investment_amount.to_string());
                event.insert("token_amount", token_amount.to_string());
                event.insert("current_stakeable_day", current_stakeable_day.to_string());
                event.insert("investment_mode", investment_mode.to_string());
                events.push(event);
            }
            LiquidityTransformerEvent::UniswapSwapResult {
                amount_token_a,
                amount_token_b,
                liquidity,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_transformer_event.type_name());
                event.insert("amount_token_a", amount_token_a.to_string());
                event.insert("amount_token_b", amount_token_b.to_string());
                event.insert("liquidity", liquidity.to_string());
                events.push(event);
            }
            LiquidityTransformerEvent::CashBackIssued {
                investor_address,
                sender_value,
                cash_back_amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_transformer_event.type_name());
                event.insert("investor_address", investor_address.to_string());
                event.insert("sender_value", sender_value.to_string());
                event.insert("cash_back_amount", cash_back_amount.to_string());
                events.push(event);
            }
            LiquidityTransformerEvent::RefundIssued {
                investor_address,
                refund_amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_transformer_event.type_name());
                event.insert("investor_address", investor_address.to_string());
                event.insert("refund_amount", refund_amount.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
