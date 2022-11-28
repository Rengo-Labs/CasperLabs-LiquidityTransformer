use std::time::SystemTime;

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
        stakeable: Key,
        scspr: Key,
        pair_stakeable: Key,
        pair_scspr: Key,
        uniswap_router: Key,
        wcspr: Key,
        amount: U512,
        time: u64,
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity_transformer.wasm",
            contract_name,
            sender,
            runtime_args! {
                "wise" => stakeable,
                "scspr" => scspr,
                "pair_wise" => pair_stakeable,
                "pair_scspr" => pair_scspr,
                "uniswap_router" => uniswap_router,
                "wcspr" => wcspr,
                "amount" => amount
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

pub const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
pub const SCSPR_AMOUNT: U512 = U512([50_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const TRANSFORMER_AMOUNT: U512 = U512([50_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const STAKEABLE_AMOUNT: U512 = U512([0, 0, 0, 0, 0, 0, 0, 0]);
pub const TWOTHOUSEND_CSPR: U512 = U512([2_000_000_000_000, 0, 0, 0, 0, 0, 0, 0]);

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => "".into(),
    }
}

pub fn session_code_call(
    env: &TestEnv,
    sender: AccountHash,
    runtime_args: RuntimeArgs,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "session-code-lt.wasm",
        "session-code-lt",
        sender,
        runtime_args,
        time,
    )
}

pub fn session_code_result<T: CLTyped + FromBytes>(
    env: &TestEnv,
    sender: AccountHash,
    key: &str,
) -> T {
    env.query_account_named_key(sender, &[key.into()])
}

pub fn deploy_uniswap_router(
    env: &TestEnv,
    owner: AccountHash,
    uniswap_factory: &TestContract,
    wcspr: &TestContract,
    uniswap_library: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-router.wasm",
        "uniswap-v2-router",
        owner,
        runtime_args! {
            "factory" => Key::Hash(uniswap_factory.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "library" => Key::Hash(uniswap_library.package_hash())
        },
        time,
    )
}

pub fn deploy_uniswap_factory(
    env: &TestEnv,
    owner: AccountHash,
    fee_to_setter: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => fee_to_setter
        },
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    contract_name: &str,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "pair-token.wasm",
        contract_name,
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply,
            "callee_package_hash" => Key::Hash(flash_swapper.package_hash()),
            "factory_hash" => Key::Hash(uniswap_factory.package_hash()),
        },
        time,
    )
}

pub fn deploy_erc20(
    env: &TestEnv,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply
        },
        time,
    )
}

pub fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        time,
    )
}

pub fn deploy_wcspr(
    env: &TestEnv,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "wcspr-token.wasm",
        "wcspr",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals
        },
        time,
    )
}

pub fn deploy_flash_swapper(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    erc20: &TestContract,
    uniswap_factory: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "flashswapper-token.wasm",
        "flash_swapper",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "dai" => Key::Hash(erc20.package_hash()),
            "uniswap_v2_factory" => Key::Hash(uniswap_factory.package_hash())
        },
        time,
    )
}

pub fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "liquidity-guard.wasm",
        "liquidity-guard",
        owner,
        runtime_args! {},
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_scspr(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    uniswap_factory: &TestContract,
    amount: U512,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "scspr.wasm",
        "scspr",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.package_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.package_hash()),
            "uniswap_factory" => Key::Hash(uniswap_factory.package_hash()),
            "amount" => amount
        },
        time,
    )
}

#[allow(clippy::new_ret_no_self, clippy::too_many_arguments)]
pub fn deploy_liquidity_transformer(
    env: &TestEnv,
    contract_name: &str,
    sender: AccountHash,
    stakeable: Key,
    scspr: Key,
    pair_stakeable: Key,
    pair_scspr: Key,
    uniswap_router: Key,
    wcspr: Key,
    amount: U512,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "liquidity_transformer.wasm",
        contract_name,
        sender,
        runtime_args! {
            "wise" => stakeable,
            "scspr" => scspr,
            "pair_wise" => pair_stakeable,
            "pair_scspr" => pair_scspr,
            "uniswap_router" => uniswap_router,
            "wcspr" => wcspr,
            "amount" => amount
        },
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_stakeable(
    env: &TestEnv,
    owner: AccountHash,
    stable_usd: &TestContract,
    scspr: &TestContract,
    wcspr: &TestContract,
    uniswap_router: &TestContract,
    uniswap_factory: &TestContract,
    uniswap_pair: &TestContract,
    liquidity_guard: &TestContract,
    amount: U512,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "stakeable-token.wasm",
        "stakeable-token",
        owner,
        runtime_args! {
            "stable_usd" => Key::Hash(stable_usd.package_hash()),
            "scspr" => Key::Hash(scspr.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.package_hash()),
            "uniswap_factory" => Key::Hash(uniswap_factory.package_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.package_hash()),
            "liquidity_guard" => Key::Hash(liquidity_guard.package_hash()),
            "amount" => amount
        },
        time,
    )
}

#[allow(clippy::type_complexity)]
pub fn deploy() -> (
    TestEnv,
    TestContract,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    u64,
) {
    let time = now();
    let env = TestEnv::new();
    let owner = env.next_user();
    let wcspr = deploy_wcspr(&env, owner, "Wrapped CSPR".into(), "WCSPR".into(), 9, time);
    let uniswap_library = deploy_uniswap_library(&env, owner, time);
    let uniswap_factory = deploy_uniswap_factory(&env, owner, Key::Account(owner), time);
    let uniswap_router = deploy_uniswap_router(
        &env,
        owner,
        &uniswap_factory,
        &wcspr,
        &uniswap_library,
        time,
    );
    let erc20 = deploy_erc20(
        &env,
        owner,
        "erc20_token".into(),
        "ERC20".into(),
        9,
        0.into(),
        time,
    );
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &erc20, &uniswap_factory, time);
    let pair_scspr: TestContract = deploy_uniswap_pair(
        &env,
        owner,
        "pair-1",
        "scspr_wcspr_pair".into(),
        "SWP".into(),
        9,
        0.into(),
        &flash_swapper,
        &uniswap_factory,
        time,
    );
    let pair_stakeable: TestContract = deploy_uniswap_pair(
        &env,
        owner,
        "pair-2",
        "stakeable_scspr_pair".into(),
        "STS".into(),
        9,
        0.into(),
        &flash_swapper,
        &uniswap_factory,
        time,
    );
    let liquidity_guard = deploy_liquidity_guard(&env, owner, time);
    let scspr = deploy_scspr(
        &env,
        owner,
        &wcspr,
        &pair_scspr,
        &uniswap_router,
        &uniswap_factory,
        SCSPR_AMOUNT,
        time,
    );
    let stakeable_token = deploy_stakeable(
        &env,
        owner,
        &erc20,
        &scspr,
        &wcspr,
        &uniswap_router,
        &uniswap_factory,
        &pair_stakeable,
        &liquidity_guard,
        STAKEABLE_AMOUNT,
        time - (2 * MILLI_SECONDS_IN_DAY), // 172800000 == 2 days in ms (launch time set in past for testing)
    );
    let liquidity_transformer = deploy_liquidity_transformer(
        &env,
        "LIQUIDITY_TRANSFORMER",
        owner,
        Key::Hash(stakeable_token.package_hash()),
        Key::Hash(scspr.package_hash()),
        Key::Hash(pair_stakeable.package_hash()),
        Key::Hash(pair_scspr.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(wcspr.package_hash()),
        TRANSFORMER_AMOUNT,
        time,
    );

    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(uniswap_router.package_hash())
        },
        time,
    );

    uniswap_router.call_contract(
        owner,
        "add_to_whitelist",
        runtime_args! {
            "address" => Key::Account(owner),
        },
        time,
    );

    uniswap_router.call_contract(
        owner,
        "add_to_whitelist",
        runtime_args! {
            "address" => Key::Hash(liquidity_transformer.package_hash()),
        },
        time,
    );

    uniswap_router.call_contract(
        owner,
        "add_to_whitelist",
        runtime_args! {
            "address" => Key::Hash(scspr.package_hash()),
        },
        time,
    );

    (
        env,
        liquidity_transformer,
        owner,
        erc20,
        wcspr,
        uniswap_router,
        pair_scspr,
        stakeable_token,
        scspr,
        uniswap_factory,
        pair_stakeable,
        flash_swapper,
        liquidity_guard,
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn forward_liquidity(
    env: &TestEnv,
    lt: &TestContract,
    owner: AccountHash,
    token: &TestContract,
    scspr: &TestContract,
    time: u64,
) -> u64 {
    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args! {
            "wise" => Key::Hash(token.package_hash())
        },
        time,
    );
    // Using session code as transformer purse fetch with access is required
    TestContract::new(
        env,
        "session-code-lt.wasm",
        "session-code-lt",
        owner,
        runtime_args! {
            "entrypoint" => "set_liquidity_transfomer",
            "package_hash" => Key::Hash(token.package_hash()),
            "immutable_transformer" => Key::Hash(lt.package_hash()),
        },
        time,
    );
    // Forward liquidity to be done after investment days
    const INVESTMENT_DAY: u64 = 20 * MILLI_SECONDS_IN_DAY;
    lt.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {},
        time + INVESTMENT_DAY,
    );
    time + INVESTMENT_DAY
}

pub fn add_liquidity(
    env: &TestEnv,
    owner: AccountHash,
    erc20: &TestContract,
    uniswap_router: &TestContract,
    uniswap_pair: &TestContract,
    wcspr: &TestContract,
    time: u64,
) {
    const AMOUNT: u128 = 100_000_000_000;
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => U256::from(AMOUNT)
        },
        time,
    );
    TestContract::new(
        env,
        "session-code-lt.wasm",
        "session-code-lt",
        owner,
        runtime_args! {
            "entrypoint" => "deposit",
            "package_hash" => Key::Hash(wcspr.package_hash()),
            "amount" => U512::from(AMOUNT),
        },
        time,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(uniswap_router.package_hash()),
            "amount" => U256::from(AMOUNT)
        },
        time,
    );
    wcspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(uniswap_router.package_hash()),
            "amount" => U512::from(AMOUNT)
        },
        time,
    );
    let deadline = time + (30 * 60 * MILLI_SECONDS_IN_DAY);
    uniswap_router.call_contract(
        owner,
        "add_liquidity",
        runtime_args! {
            "token_a" => Key::Hash(erc20.package_hash()),
            "token_b" => Key::Hash(wcspr.package_hash()),
            "amount_a_desired" => U256::from(10_000_000_000_u128),
            "amount_b_desired" => U256::from(10_000_000_000_u128),
            "amount_a_min" => U256::from(1_000_000_000_u128),
            "amount_b_min" => U256::from(1_000_000_000_u128),
            "to" => Key::Hash(uniswap_pair.package_hash()),
            "pair" => Some(Key::Hash(uniswap_pair.package_hash())),
            "deadline" => U256::from(deadline),
        },
        time,
    );
}
