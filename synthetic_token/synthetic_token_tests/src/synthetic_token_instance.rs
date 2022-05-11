use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct SYNTHETICTOKENInstance(TestContract);

impl SYNTHETICTOKENInstance {
    pub fn instance(scspr: TestContract) -> SYNTHETICTOKENInstance {
        SYNTHETICTOKENInstance(scspr)
    }

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

    // Result methods
    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }

    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }
}
