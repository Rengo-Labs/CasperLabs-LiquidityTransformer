use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct SYNTHETICTOKENInstance(TestContract);

impl SYNTHETICTOKENInstance {
    pub fn instance(scspr: TestContract) -> SYNTHETICTOKENInstance {
        SYNTHETICTOKENInstance(scspr)
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        wcspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "synthetic_token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "wcspr" => wcspr,
                "uniswap_pair" => uniswap_pair,
                "uniswap_router" => uniswap_router,
            },
            0,
        )
    }

    pub fn get_trading_fee_amount(
        &self,
        sender: AccountHash,
        previous_evaluation: U256,
        current_evaluation: U256,
    ) {
        self.0.call_contract(
            sender,
            "get_trading_fee_amount",
            runtime_args! {
                "previous_evaluation" => previous_evaluation,
                "current_evaluation" => current_evaluation
            },
            0,
        );
    }

    pub fn get_amount_payout(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "get_amount_payout",
            runtime_args! {
                "amount" => amount
            },
            0,
        );
    }

    pub fn get_wrapped_balance(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_wrapped_balance", runtime_args! {}, 0);
    }

    pub fn get_synthetic_balance(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_synthetic_balance", runtime_args! {}, 0);
    }

    pub fn get_evaluation(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_evaluation", runtime_args! {}, 0);
    }

    pub fn get_pair_balances(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_pair_balances", runtime_args! {}, 0);
    }

    pub fn get_lp_token_balance(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_lp_token_balance", runtime_args! {}, 0);
    }

    pub fn get_liquidity_percent(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_liquidity_percent", runtime_args! {}, 0);
    }

    // Result methods
    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }

    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }
}
