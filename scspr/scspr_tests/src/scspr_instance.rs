use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256, U512
};
use test_env::{TestContract, TestEnv};

pub struct SCSPRInstance(TestContract);

impl SCSPRInstance {
    pub fn instance(scspr: TestContract) -> SCSPRInstance {
        SCSPRInstance(scspr)
    }

    #[allow(clippy::new_ret_no_self, clippy::too_many_arguments)]
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        wcspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
        uniswap_factory: Key,
        amount: U512
    ) -> TestContract {
        TestContract::new(
            env,
            "scspr.wasm",
            contract_name,
            sender,
            runtime_args! {
                "wcspr" => wcspr,
                "uniswap_pair" => uniswap_pair,
                "uniswap_router" => uniswap_router,
                "uniswap_factory" => uniswap_factory,
                "amount" => amount
            },
            0,
        )
    }

    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        scspr: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "proxy-scspr.wasm",
            contract_name,
            sender,
            runtime_args! {
                "scspr" => scspr,
            },
            0,
        )
    }

    pub fn constructor(
        &self,
        sender: AccountHash,
        erc20: Key,
        uniswap_factory: Key,
        synthetic_helper: Key,
        synthetic_token: Key,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "erc20" => erc20,
                "uniswap_factory" => uniswap_factory,
                "synthetic_helper" => synthetic_helper,
                "synthetic_token" => synthetic_token,
            },
            0,
        );
    }

    pub fn scspr(&self) -> Key {
        let tmp: ContractPackageHash = self.0.query_named_key(String::from("self_package_hash"));
        Key::from(tmp)
    }

    pub fn deposit(&self, sender: AccountHash, msg_value: U256, succesor_purse: URef) {
        self.0.call_contract(
            sender,
            "deposit",
            runtime_args! {
                "msg_value" => msg_value,
                "succesor_purse" => succesor_purse
            },
            0,
        );
    }

    pub fn withdraw(&self, sender: AccountHash, msg_value: U256, succesor_purse: URef) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "msg_value" => msg_value,
                "succesor_purse" => succesor_purse
            },
            0,
        );
    }

    pub fn define_helper(&self, sender: AccountHash, transfer_helper: Key) {
        self.0.call_contract(
            sender,
            "define_helper",
            runtime_args! {
                "transfer_helper" => transfer_helper
            },
            0,
        );
    }

    pub fn define_token(&self, sender: AccountHash, wise_token: Key) {
        self.0.call_contract(
            sender,
            "define_token",
            runtime_args! {
                "wise_token" => wise_token
            },
            0,
        );
    }

    pub fn create_pair(&self, sender: AccountHash, pair: Key) {
        self.0.call_contract(
            sender,
            "create_pair",
            runtime_args! {
                "pair" => pair
            },
            0,
        );
    }

    pub fn forward_ownership(&self, sender: AccountHash, new_master: Key) {
        self.0.call_contract(
            sender,
            "forward_ownership",
            runtime_args! {
                "new_master" => new_master
            },
            0,
        );
    }

    pub fn set_wise(&self, sender: AccountHash, wise: Key) {
        self.0.call_contract(
            sender,
            "set_wise",
            runtime_args! {
                "wise" => wise
            },
            0,
        );
    }

    pub fn temp_purse(&self, sender: AccountHash, contract: Key) {
        self.0.call_contract(
            sender,
            "temp_purse",
            runtime_args! {
                "contract" => contract
            },
            0,
        );
    }

    // Transformer
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

    // Result methods
    pub fn form_liquidity_result(&self) -> bool {
        self.0.query_named_key("form_liquidity_result".to_string())
    }

    pub fn define_helper_result(&self) -> Key {
        self.0.query_named_key("define_helper_result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }

    pub fn proxy_package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("package_hash".to_string())
    }

    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }
}
