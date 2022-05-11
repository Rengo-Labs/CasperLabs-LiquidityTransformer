use core::str::FromStr;

use crate::alloc::string::ToString;
use crate::data;
use crate::erc20_crate::ERC20;
use crate::error::Error;
use crate::event::SyntheticTokenEvent;
use crate::synthetic_helper_crate::SYNTHETICHELPER;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use contract_utils::{ContractContext, ContractStorage};
use synthetic_helper_crate::data::*;

pub trait SYNTHETICTOKEN<Storage: ContractStorage>:
    ContractContext<Storage> + SYNTHETICHELPER<Storage> + ERC20<Storage>
{
    fn init(
        &mut self,
        wcspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        master_address_purse: URef,
    ) {
        ERC20::init(
            self,
            "Synthetic Token".to_string(),
            "ST".to_string(),
            18,
            0.into(),
            "".to_string(),
            "".to_string(),
            contract_hash,
            package_hash,
        );
        // DEFAULT INITIALIZATIONS
        data::set_owner(self.get_caller());

        data::set_master_address(self.get_caller());
        data::set_master_address_purse(master_address_purse);
        data::set_current_evaluation(0.into());

        data::set_token_defined(false);
        data::set_allow_deposit(false);
        data::set_helper_defined(false);
        data::set_bypass_enabled(false);

        data::set_uniswap_router(uniswap_router);
        data::set_uniswap_pair(uniswap_pair);
        data::set_wcspr(wcspr);

        data::set_contract_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_self_purse(system::create_purse());
    }

    fn get_trading_fee_amount(
        &mut self,
        previous_evaluation: U256,
        current_evaluation: U256,
    ) -> U256 {
        self._get_trading_fee_amount(previous_evaluation, current_evaluation)
    }

    fn _get_trading_fee_amount(
        &mut self,
        previous_evaluation: U256,
        current_evaluation: U256,
    ) -> U256 {
        let ratio_amount: U256 = previous_evaluation
            .checked_mul(PRECISION_POINTS_POWER4)
            .unwrap_or_revert()
            .checked_div(current_evaluation)
            .ok_or(ApiError::from(Error::Div1))
            .unwrap_or_revert();

        let recipient_amount = self
            ._get_synthetic_balance()
            .checked_mul(PRECISION_POINTS_POWER2)
            .unwrap_or_revert()
            .checked_div(self._get_wrapped_balance())
            .ok_or(ApiError::from(Error::Div2))
            .unwrap_or_revert();

        let difference = PRECISION_POINTS_POWER2
            .checked_sub(ratio_amount.integer_sqrt())
            .ok_or(ApiError::from(Error::Sub1))
            .unwrap_or_revert()
            .checked_mul(recipient_amount.integer_sqrt())
            .unwrap_or_revert()
            .checked_mul(self._get_lp_token_balance())
            .unwrap_or_revert()
            .checked_div(self._get_liquidity_percent())
            .ok_or(ApiError::from(Error::Div3))
            .unwrap_or_revert();

        difference
            .checked_div(PRECISION_POINTS)
            .ok_or(ApiError::from(Error::Div4))
            .unwrap_or_revert()
    }

    fn get_amount_payout(&mut self, amount: U256) -> U256 {
        self._get_amount_payout(amount)
    }

    fn _get_amount_payout(&mut self, amount: U256) -> U256 {
        let product: U256 = amount
            .checked_mul(self._get_liquidity_percent())
            .unwrap_or_revert()
            .checked_mul(PRECISION_POINTS)
            .unwrap_or_revert();

        let quotient: U256 = product
            .checked_mul(self._get_lp_token_balance())
            .unwrap_or_revert()
            .checked_div(self._get_wrapped_balance())
            .ok_or(ApiError::from(Error::Div6))
            .unwrap_or_revert();

        quotient
            .checked_div(PRECISION_POINTS_POWER3)
            .ok_or(ApiError::from(Error::Div7))
            .unwrap_or_revert()
    }

    fn get_wrapped_balance(&mut self) -> U256 {
        self._get_wrapped_balance()
    }

    fn _get_wrapped_balance(&mut self) -> U256 {
        self._get_balance_of(data::get_uniswap_pair(), data::get_wcspr())
    }

    fn get_synthetic_balance(&mut self) -> U256 {
        self._get_synthetic_balance()
    }

    fn _get_synthetic_balance(&mut self) -> U256 {
        self._get_balance_of(
            Key::from(data::get_package_hash()),
            data::get_uniswap_pair(),
        )
    }

    fn get_evaluation(&mut self) -> U256 {
        self._get_evaluation()
    }

    fn _get_evaluation(&mut self) -> U256 {
        let liquidity_percent: U256 = self._get_liquidity_percent();
        let liquidity_percent_squared = liquidity_percent
            .checked_mul(liquidity_percent)
            .unwrap_or_revert();

        self._get_wrapped_balance()
            .checked_mul(PRECISION_POINTS_POWER4)
            .unwrap_or_revert()
            .checked_mul(self._get_synthetic_balance())
            .unwrap_or_revert()
            .checked_div(liquidity_percent_squared)
            .ok_or(ApiError::from(Error::Div8))
            .unwrap_or_revert()
    }

    fn get_pair_balances(&mut self) -> (U256, U256) {
        (self._get_wrapped_balance(), self._get_synthetic_balance())
    }

    fn get_lp_token_balance(&mut self) -> U256 {
        self._get_lp_token_balance()
    }

    fn _get_lp_token_balance(&mut self) -> U256 {
        self._get_balance_of(
            data::get_uniswap_pair(),
            Key::from(data::get_package_hash()),
        )
    }

    fn get_liquidity_percent(&mut self) -> U256 {
        self._get_liquidity_percent()
    }

    fn _get_liquidity_percent(&mut self) -> U256 {
        let total_supply: U256 = runtime::call_versioned_contract(
            data::get_uniswap_pair().into_hash().unwrap().into(),
            None,
            "total_supply",
            runtime_args! {},
        );
        total_supply
            .checked_mul(PRECISION_POINTS_POWER2)
            .unwrap_or_revert()
            .checked_div(self._get_lp_token_balance())
            .ok_or(ApiError::from(Error::Div5))
            .unwrap_or_revert()
    }

    fn _fees_decision(&mut self) {
        let previous_evaluation: U256 = data::get_current_evaluation();
        let new_evaluation = self._get_evaluation();

        let previous_condition: U256 = previous_evaluation * TRADING_FEE_CONDITION;

        let new_condition = new_evaluation * EQUALIZE_SIZE_VALUE;

        if new_condition > previous_condition {
            self._extract_and_send_fees(previous_evaluation, new_evaluation);
        }
    }

    fn _extract_and_send_fees(&mut self, previous_evaluation: U256, current_evaluation: U256) {
        let _get_trading_fee_amount: U256 =
            self._get_trading_fee_amount(previous_evaluation, current_evaluation);
        let (amount_wcspr, amount_scspr): (U256, U256) =
            self._remove_liquidity(_get_trading_fee_amount);

        self.synthetic_token_emit(&SyntheticTokenEvent::LiquidityRemoved {
            amount_wcspr,
            amount_scspr,
        });

        self._unwrap(amount_wcspr);
        self._profit(amount_wcspr);

        self.burn(Key::from(data::get_package_hash()), amount_scspr);

        let master_address: Key = data::get_master_address();
        self.synthetic_token_emit(&SyntheticTokenEvent::SendFeesToMaster {
            amount_wcspr,
            master_address,
        });
    }

    fn _swap_exact_tokens_for_tokens(
        &mut self,
        amount: U256,
        amount_out_min: U256,
        from_token_address: Key,
        to_token_address: Key,
    ) -> U256 {
        let path: Vec<Key> = self._prepare_path(from_token_address, to_token_address);
        let mut time: u64 = runtime::get_blocktime().into();
        time += 7200;
        let ret: Vec<U256> = runtime::call_versioned_contract(
            data::get_uniswap_router().into_hash().unwrap().into(),
            None,
            "swap_exact_tokens_for_tokens",
            runtime_args! {
                "amount" => amount,
                "amount_out_min" => amount_out_min,
                "path" => path,
                "transfer_helper" => data::get_transfer_helper(),
                "time" => U256::from(time)
            },
        );
        ret[0]
    }

    fn _add_liquidity(&mut self, _amount_wcspr: U256, _amount_scspr: U256) -> (U256, U256) {
        let () = runtime::call_versioned_contract(
            data::get_wcspr().into_hash().unwrap().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => data::get_uniswap_router(),
                "amount" => _amount_wcspr
            },
        );
        self.approve(data::get_uniswap_router(), _amount_scspr);

        let mut time: u64 = runtime::get_blocktime().into();
        time += 7200;
        let (amount_wcspr, amount_scspr, liquidity): (U256, U256, U256) =
            runtime::call_versioned_contract(
                data::get_uniswap_router().into_hash().unwrap().into(),
                None,
                "add_liquidity",
                runtime_args! {
                    "token_a" => data::get_wcspr(),
                    "token_b" => Key::from(data::get_package_hash()),
                    "amount_a_desired" => _amount_wcspr,
                    "amount_b_desired" => _amount_scspr,
                    "amount_a_min" => U256::from(0),
                    "amount_b_min" => U256::from(0),
                    "to" => Key::from(data::get_package_hash()),
                    "deadline" => U256::from(time),
                    "pair" => Some(data::get_uniswap_pair())
                },
            );

        self.synthetic_token_emit(&SyntheticTokenEvent::LiquidityAdded {
            amount_wcspr,
            amount_scspr,
            liquidity,
        });

        (amount_wcspr, amount_scspr)
    }

    fn _remove_liquidity(&mut self, amount: U256) -> (U256, U256) {
        let () = runtime::call_versioned_contract(
            data::get_uniswap_pair().into_hash().unwrap().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => data::get_uniswap_router(),
                "amount" => amount
            },
        );
        let mut time: u64 = runtime::get_blocktime().into();
        time += 7200;

        let (amount_wcspr, amount_scspr): (U256, U256) = runtime::call_versioned_contract(
            data::get_uniswap_router().into_hash().unwrap().into(),
            None,
            "remove_liquidity",
            runtime_args! {
                "token_a" => data::get_wcspr(),
                "token_b" => Key::from(data::get_package_hash()),
                "liquidity" => amount,
                "amount_a_min" => U256::from(0),
                "amount_b_min" => U256::from(0),
                "to" => Key::from(data::get_package_hash()),
                "deadline" => U256::from(time)
            },
        );

        (amount_wcspr, amount_scspr)
    }

    fn _profit_arbitrage_remove(&mut self) -> U256 {
        let wrapped_balance: U256 = self._get_wrapped_balance();
        let synthetic_balance: U256 = self._get_synthetic_balance();

        let product: U256 = wrapped_balance * synthetic_balance;

        let get_double_root: U256 = self._get_double_root(product);
        let difference: U256 = ((wrapped_balance + synthetic_balance) - get_double_root)
            * self._get_lp_token_balance();

        (((difference * self._get_liquidity_percent()) / wrapped_balance)
            * LIQUIDITY_PERCENTAGE_CORRECTION)
            / PRECISION_POINTS_POWER3
    }

    fn _to_remove_cspr(&mut self) -> U256 {
        let wrapped_balance: U256 = self._get_wrapped_balance();
        let product_a: U256 = wrapped_balance.integer_sqrt() * PRECISION_DIFF;
        let product_b: U256 = self._get_synthetic_balance() * PRECISION_POINTS_POWER4;
        let difference: U256 = product_b.integer_sqrt() - product_a;
        let quotient: U256 = (wrapped_balance.integer_sqrt() * PRECISION_PROD) / difference;
        ((((PRECISION_POINTS_POWER2 - quotient) - self._get_liquidity_percent())
            * self._get_lp_token_balance())
            * LIQUIDITY_PERCENTAGE_CORRECTION)
            / PRECISION_POINTS_POWER5
    }

    fn _swap_amount_arbitrage_scspr(&mut self) -> U256 {
        let product: U256 = self._get_synthetic_balance() * self._get_wrapped_balance();
        let difference = product.integer_sqrt() - self._get_synthetic_balance();

        (difference * PRECISION_FEES_PROD) / PRECISION_POINTS_POWER3
    }

    fn _self_burn(&mut self) {
        let get_balance_of: U256 = self._get_balance_of(
            Key::from(data::get_package_hash()),
            Key::from(data::get_package_hash()),
        );

        self.burn(Key::from(data::get_package_hash()), get_balance_of);
    }

    fn _clean_up(&mut self, deposit_amount: U256) {
        self._skim_pair();
        self._self_burn();
        let amount_wcspr: U256 =
            U256::from_dec_str(self._get_balance_diff(deposit_amount).to_string().as_str())
                .unwrap();
        self._profit(amount_wcspr);
    }

    fn _unwrap(&mut self, amount_wcspr: U256) {
        let amount_wcspr: U512 = U512::from_str(amount_wcspr.to_string().as_str()).unwrap();

        data::set_bypass_enabled(true);

        let _: Result<(), u32> = runtime::call_versioned_contract(
            data::get_wcspr().into_hash().unwrap().into(),
            None,
            "withdraw",
            runtime_args! {
                "to_purse" => data::get_self_purse(),
                "amount" => amount_wcspr
            },
        );

        data::set_bypass_enabled(false);
    }

    fn _profit(&mut self, amount_wcspr: U256) {
        let ret = system::transfer_from_purse_to_purse(
            data::get_self_purse(),
            data::get_master_address_purse(),
            U512::from_str(amount_wcspr.to_string().as_str()).unwrap(),
            None,
        );

        if ret.is_err() {
            runtime::revert(ret.err().unwrap_or_revert());
        }

        self.synthetic_token_emit(&SyntheticTokenEvent::MasterProfit {
            amount_wcspr,
            master_address: data::get_master_address(),
        });
    }

    fn _update_evaluation(&mut self) {
        data::set_current_evaluation(self._get_evaluation());
    }

    fn _skim_pair(&mut self) {
        let () = runtime::call_versioned_contract(
            data::get_uniswap_pair()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "skim",
            runtime_args! {
                "to" => data::get_master_address()
            },
        );
    }

    fn _arbitrage_decision(&mut self) {
        let wrapped_balance: U256 = self._get_wrapped_balance();
        let synthetic_balance: U256 = self._get_synthetic_balance();

        if wrapped_balance < synthetic_balance {
            self._arbitrage_cspr(wrapped_balance, synthetic_balance);
        }

        if wrapped_balance > synthetic_balance {
            self._arbitrage_scspr(wrapped_balance, synthetic_balance);
        }
    }

    fn _arbitrage_scspr(&mut self, wrapped_balance: U256, synthetic_balance: U256) {
        let condition_wcspr: U256 = wrapped_balance * PRECISION_POINTS;
        let condition_scspr: U256 = synthetic_balance * ARBITRAGE_CONDITION;
        if condition_wcspr <= condition_scspr {
            return;
        }
        let amount = self._profit_arbitrage_remove();
        let (amount_wcspr, amount_scspr): (U256, U256) = self._remove_liquidity(amount);

        self.synthetic_token_emit(&SyntheticTokenEvent::LiquidityRemoved {
            amount_wcspr,
            amount_scspr,
        });

        self._unwrap(amount_wcspr);
        self._profit(amount_wcspr);
        self.mint(Key::from(data::get_package_hash()), LIMIT_AMOUNT);

        let swap_amount: U256 = self._swap_amount_arbitrage_scspr();

        let () = runtime::call_versioned_contract(
            data::get_wcspr().into_hash().unwrap().into(),
            None,
            "approve",
            runtime_args! {
                "owner" => Key::from(data::get_package_hash()),
                "spender" => data::get_uniswap_router(),
                "amount" => swap_amount
            },
        );

        let amount_out_received_wcspr: U256 = self._swap_exact_tokens_for_tokens(
            swap_amount,
            0.into(),
            Key::from(data::get_package_hash()),
            data::get_wcspr(),
        );

        let () = runtime::call_versioned_contract(
            data::get_transfer_helper().into_hash().unwrap().into(),
            None,
            "forward_funds",
            runtime_args! {
                "to" => data::get_wcspr(),
                "amount" => amount_out_received_wcspr
            },
        );

        let get_balance_of: U256 = self._get_balance_of(
            Key::from(data::get_package_hash()),
            Key::from(data::get_package_hash()),
        );

        self._add_liquidity(amount_out_received_wcspr, get_balance_of);

        self._self_burn();

        let master_address: Key = data::get_master_address();
        self.synthetic_token_emit(&SyntheticTokenEvent::SendArbitrageProfitToMaster {
            amount_wcspr,
            master_address,
        });
    }

    fn _arbitrage_cspr(&mut self, wrapped_balance: U256, synthetic_balance: U256) {
        let condition_wcspr: U256 = wrapped_balance * ARBITRAGE_CONDITION;
        let condition_scspr = synthetic_balance * PRECISION_POINTS;

        if condition_wcspr >= condition_scspr {
            return;
        }

        let amount = self._profit_arbitrage_remove();
        let (amount_wcspr, amount_scspr) = self._remove_liquidity(amount);

        self.synthetic_token_emit(&SyntheticTokenEvent::LiquidityRemoved {
            amount_wcspr,
            amount_scspr,
        });

        self._unwrap(amount_wcspr);
        self._profit(amount_wcspr);

        let amount = self._to_remove_cspr();
        let (amount_wcspr, amount_scspr) = self._remove_liquidity(amount);

        self.synthetic_token_emit(&SyntheticTokenEvent::LiquidityRemoved {
            amount_wcspr,
            amount_scspr,
        });

        let () = runtime::call_versioned_contract(
            data::get_wcspr().into_hash().unwrap().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => data::get_uniswap_router(),
                "amount" => LIMIT_AMOUNT
            },
        );

        let () = runtime::call_versioned_contract(
            data::get_wcspr().into_hash().unwrap().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => data::get_uniswap_router(),
                "amount" => amount_wcspr
            },
        );

        let amount_out_received_scspr = self._swap_exact_tokens_for_tokens(
            amount_wcspr,
            0.into(),
            data::get_wcspr(),
            Key::from(data::get_package_hash()),
        );

        let () = runtime::call_versioned_contract(
            data::get_transfer_helper().into_hash().unwrap().into(),
            None,
            "forward_funds",
            runtime_args! {
                "to" => Key::from(data::get_package_hash()),
                "amount" => amount_out_received_scspr
            },
        );

        self._self_burn();

        let master_address: Key = data::get_master_address();
        self.synthetic_token_emit(&SyntheticTokenEvent::SendArbitrageProfitToMaster {
            amount_wcspr,
            master_address,
        });
    }

    fn synthetic_token_emit(&mut self, synthetic_token_event: &SyntheticTokenEvent) {
        let mut events = Vec::new();
        let tmp = data::get_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split("-").collect();
        let package_hash = tmp[1].to_string();
        match synthetic_token_event {
            SyntheticTokenEvent::LiquidityRemoved {
                amount_wcspr,
                amount_scspr,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", synthetic_token_event.type_name());
                event.insert("amount_wcspr", amount_wcspr.to_string());
                event.insert("amount_scspr", amount_scspr.to_string());
                events.push(event);
            }
            SyntheticTokenEvent::SendFeesToMaster {
                amount_wcspr,
                master_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", synthetic_token_event.type_name());
                event.insert("amount_wcspr", amount_wcspr.to_string());
                event.insert("master_address", master_address.to_string());
                events.push(event);
            }
            SyntheticTokenEvent::LiquidityAdded {
                amount_wcspr,
                amount_scspr,
                liquidity,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", synthetic_token_event.type_name());
                event.insert("amount_wcspr", amount_wcspr.to_string());
                event.insert("amount_scspr", amount_scspr.to_string());
                event.insert("liquidity", liquidity.to_string());
                events.push(event);
            }
            SyntheticTokenEvent::MasterProfit {
                amount_wcspr,
                master_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", synthetic_token_event.type_name());
                event.insert("amount_wcspr", amount_wcspr.to_string());
                event.insert("master_address", master_address.to_string());
                events.push(event);
            }
            SyntheticTokenEvent::SendArbitrageProfitToMaster {
                amount_wcspr,
                master_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", synthetic_token_event.type_name());
                event.insert("amount_wcspr", amount_wcspr.to_string());
                event.insert("master_address", master_address.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
