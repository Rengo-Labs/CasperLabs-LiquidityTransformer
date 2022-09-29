use std::time::SystemTime;

use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256, U512};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::synthetic_token_instance::SYNTHETICTOKENInstance;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn deploy_fund_contract_purse_proxy(
    env: &TestEnv,
    sender: AccountHash,
    destination_package_hash: Key,
    destination_entrypoint: &str,
    amount: U512,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "purse-proxy.wasm",
        "purse-proxy",
        sender,
        runtime_args! {
            "destination_package_hash" => destination_package_hash,
            "destination_entrypoint" => destination_entrypoint,
            "amount" => amount,
        },
        time,
    )
}

fn deploy_uniswap_factory(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => Key::from(owner)
        },
        time,
    )
}

fn deploy_wcspr(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    let decimals: u8 = 18;
    TestContract::new(
        env,
        "wcspr-token.wasm",
        "wcspr",
        owner,
        runtime_args! {
            "name" => "Wrapper Casper",
            "symbol" => "WCSPR",
            "decimals" => decimals
        },
        time,
    )
}

fn deploy_flash_swapper(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
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
            "dai" => Key::Hash(wcspr.package_hash()),
            "uniswap_v2_factory" => Key::Hash(uniswap_factory.package_hash())
        },
        time,
    )
}

fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
    time: u64,
) -> TestContract {
    let flash_swapper_package_hash: Key =
        flash_swapper.query_named_key("contract_package_hash".to_string());
    TestContract::new(
        env,
        "pair-token.wasm",
        "Pair",
        owner,
        runtime_args! {
            "name" => "pair",
            "symbol" => "PAIR",
            "decimals" => 18_u8,
            "initial_supply" => U256::from(0),
            "callee_package_hash" => flash_swapper_package_hash,
            "factory_hash" => Key::Hash(uniswap_factory.package_hash()),
        },
        time,
    )
}

fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        time,
    )
}

fn deploy_uniswap_router(
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

fn deploy() -> (
    TestEnv,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let wcspr = deploy_wcspr(&env, owner, now());
    let uniswap_library = deploy_uniswap_library(&env, owner, now());
    let uniswap_factory = deploy_uniswap_factory(&env, owner, now());
    let uniswap_router = deploy_uniswap_router(
        &env,
        owner,
        &uniswap_factory,
        &wcspr,
        &uniswap_library,
        now(),
    );
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &uniswap_factory, now());
    let uniswap_pair: TestContract =
        deploy_uniswap_pair(&env, owner, &flash_swapper, &uniswap_factory, now());

    let synthetic_token = SYNTHETICTOKENInstance::new(
        &env,
        "synthetic_token",
        owner,
        Key::Hash(wcspr.package_hash()),
        Key::Hash(uniswap_pair.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        now(),
    );

    (
        env,
        owner,
        synthetic_token,
        wcspr,
        uniswap_library,
        uniswap_factory,
        uniswap_router,
        flash_swapper,
        uniswap_pair,
    )
}

#[test]
fn test_synthetic_token_deploy() {
    let (_, _, _, _, _, _, _, _, _) = deploy();
}

#[test]
fn test_get_wrapped_balance() {
    let (_, owner, synthetic_token, _, _, _, _, _, _) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token);
    instance.get_wrapped_balance(owner, now());
}

#[test]
fn test_get_synthetic_balance() {
    let (_, owner, synthetic_token, _, _, _, _, _, _) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token);
    instance.get_synthetic_balance(owner, now());
}

#[test]
fn test_get_evaluation() {
    let (_, owner, synthetic_token, _, _, _, _, _, uniswap_pair) = deploy();
    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(synthetic_token.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );
    synthetic_token.call_contract(owner, "get_evaluation", runtime_args! {}, 0);
}

#[test]
fn test_get_pair_balances() {
    let (_, owner, synthetic_token, _, _, _, _, _, _) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token);
    instance.get_pair_balances(owner, now());
}

#[test]
fn test_get_lp_token_balance() {
    let (_, owner, synthetic_token, _, _, _, _, _, _) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token);
    instance.get_lp_token_balance(owner, now());
}

#[test]
fn test_get_liquidity_percent() {
    let (_, owner, synthetic_token, _, _, _, _, _, uniswap_pair) = deploy();
    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(synthetic_token.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );
    synthetic_token.call_contract(owner, "get_liquidity_percent", runtime_args! {}, 0);
}
