use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, URef,
    U256, U512,
};
use casperlabs_test_env::{TestContract, TestEnv};

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
                "amount" => U512::from(0)
            },
            0,
        )
    }

    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        liquidity_transformer: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "proxy-liquidity-transformer.wasm",
            contract_name,
            sender,
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer,
            },
            0,
        )
    }

    pub fn reserve_wise(
        &self,
        sender: AccountHash,
        liquidity_transformer: Key,
        investment_mode: u8,
        msg_value: U256,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "reserve_wise",
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer,
                "investment_mode" => investment_mode,
                "msg_value" => msg_value,
            },
            block_time,
        );
    }

    pub fn reserve_wise_with_token(
        &self,
        sender: AccountHash,
        proxy: Key,
        token_address: Key,
        token_amount: U256,
        investment_mode: u8,
        block_time: u64,
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
            block_time,
        );
    }

    pub fn forward_liquidity(&self, sender: AccountHash, purse: URef) {
        self.0.call_contract(
            sender,
            "forward_liquidity",
            runtime_args! {
                "purse" => purse
            },
            0,
        );
    }

    pub fn get_my_tokens(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_my_tokens", runtime_args! {}, 0);
    }

    pub fn payout_investor_address(&self, sender: AccountHash, investor_address: Key) {
        self.0.call_contract(
            sender,
            "payout_investor_address",
            runtime_args! {
                "investor_address" => investor_address
            },
            0,
        );
    }

    pub fn prepare_path(&self, sender: AccountHash, token_address: Key) {
        self.0.call_contract(
            sender,
            "prepare_path",
            runtime_args! {
                "token_address" => token_address,
            },
            0,
        );
    }

    pub fn current_stakeable_day(&self, sender: AccountHash, block_time: u64) {
        self.0.call_contract(
            sender,
            "current_stakeable_day",
            runtime_args! {},
            block_time,
        );
    }

    pub fn request_refund(
        &self,
        sender: AccountHash,
        liquidity_transformer: Key,
        proxy_key: Key,
        block_time: u64,
    ) {
        self.0.call_contract(
            sender,
            "request_refund",
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer,
                "proxy_key" => proxy_key
            },
            block_time,
        );
    }

    pub fn approve(&self, sender: AccountHash, token_address: Key, spender: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "token_address" => token_address,
                "spender" => spender,
                "amount" => amount
            },
            0,
        );
    }

    pub fn temp_purse(&self, sender: AccountHash, liquidity_transformer: Key) {
        self.0.call_contract(
            sender,
            "temp_purse",
            runtime_args! {
                "liquidity_transformer" => liquidity_transformer
            },
            0,
        );
    }

    // Result method
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }
}
