use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, URef, U256,
};
use std::time::{self, SystemTime};
use test_env::{TestContract, TestEnv};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub struct LIQUIDITYTRANSFORMERInstance(TestContract);

impl LIQUIDITYTRANSFORMERInstance {
    pub fn instance(liquidity_transformer: TestContract) -> LIQUIDITYTRANSFORMERInstance {
        LIQUIDITYTRANSFORMERInstance(liquidity_transformer)
    }

    #[allow(clippy::new_ret_no_self, clippy::too_many_arguments)]
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        wise_token: Key,
        scspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
        wcspr: Key,
        time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity_transformer.wasm",
            contract_name,
            sender,
            runtime_args! {
                "wise_token" => wise_token,
                "scspr" => scspr,
                "uniswap_pair" => uniswap_pair,
                "uniswap_router" => uniswap_router,
                "wcspr" => wcspr,
            },
            time,
        )
    }

    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        liquidity_transformer: Key,
        time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "proxy-liquidity-transformer.wasm",
            contract_name,
            sender,
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer,
            },
            time,
        )
    }

    pub fn reserve_wise(
        &self,
        sender: AccountHash,
        liquidity_transformer: Key,
        investment_mode: u8,
        msg_value: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "reserve_wise",
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer,
                "investment_mode" => investment_mode,
                "msg_value" => msg_value,
            },
            time,
        );
    }

    pub fn reserve_wise_with_token(
        &self,
        sender: AccountHash,
        proxy: Key,
        token_address: Key,
        token_amount: U256,
        investment_mode: u8,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "reserve_wise_with_token",
            runtime_args! {
                "proxy" => proxy,
                "token_address" => token_address,
                "token_amount" => token_amount,
                "investment_mode" => investment_mode
            },
            time,
        );
    }

    pub fn forward_liquidity(&self, sender: AccountHash, purse: URef, time: u64) {
        self.0.call_contract(
            sender,
            "forward_liquidity",
            runtime_args! {
                "purse" => purse
            },
            time,
        );
    }

    pub fn get_my_tokens(&self, sender: AccountHash, time: u64) {
        self.0
            .call_contract(sender, "get_my_tokens", runtime_args! {}, time);
    }

    pub fn payout_investor_address(&self, sender: AccountHash, investor_address: Key, time: u64) {
        self.0.call_contract(
            sender,
            "payout_investor_address",
            runtime_args! {
                "investor_address" => investor_address
            },
            time,
        );
    }

    pub fn prepare_path(&self, sender: AccountHash, token_address: Key, time: u64) {
        self.0.call_contract(
            sender,
            "prepare_path",
            runtime_args! {
                "token_address" => token_address,
            },
            time,
        );
    }

    pub fn current_stakeable_day(&self, sender: AccountHash, time: u64) {
        self.0
            .call_contract(sender, "current_stakeable_day", runtime_args! {}, time);
    }

    pub fn request_refund(
        &self,
        sender: AccountHash,
        liquidity_transformer: Key,
        proxy_key: Key,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "request_refund",
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer,
                "proxy_key" => proxy_key
            },
            time,
        );
    }

    pub fn approve(
        &self,
        sender: AccountHash,
        token_address: Key,
        spender: Key,
        amount: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "token_address" => token_address,
                "spender" => spender,
                "amount" => amount
            },
            time,
        );
    }

    pub fn temp_purse(&self, sender: AccountHash, liquidity_transformer: Key, time: u64) {
        self.0.call_contract(
            sender,
            "temp_purse",
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer
            },
            time,
        );
    }

    // Result method
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }
}
