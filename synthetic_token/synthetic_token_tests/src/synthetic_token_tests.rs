use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::synthetic_token_instance::SYNTHETICTOKENInstance;

pub fn deploy_fund_contract_purse_proxy(
    env: &TestEnv,
    sender: AccountHash,
    destination_package_hash: Key,
    destination_entrypoint: &str,
    amount: U512,
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
        0,
    )
}

fn deploy_uniswap_factory(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => Key::from(owner)
        },
        0,
    )
}

fn deploy_wcspr(env: &TestEnv, owner: AccountHash) -> TestContract {
    let decimals: u8 = 18;
    TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        owner,
        runtime_args! {
            "name" => "Wrapper Casper",
            "symbol" => "WCSPR",
            "decimals" => decimals
        },
        0,
    )
}

fn deploy_flash_swapper(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    uniswap_factory: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "flashswapper-token.wasm",
        "flash_swapper",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "dai" => Key::Hash(wcspr.package_hash()),
            "uniswap_v2_factory" => Key::Hash(uniswap_factory.package_hash())
        },
        0,
    )
}

fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
) -> TestContract {
    let flash_swapper_package_hash: Key =
        flash_swapper.query_named_key("contract_package_hash".to_string());
    TestContract::new(
        &env,
        "pair-token.wasm",
        "Pair",
        owner,
        runtime_args! {
            "name" => "pair",
            "symbol" => "PAIR",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(0),
            "callee_package_hash" => flash_swapper_package_hash,
            "factory_hash" => Key::Hash(uniswap_factory.package_hash()),
        },
        0,
    )
}

fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        0,
    )
}

fn deploy_uniswap_router(
    env: &TestEnv,
    owner: AccountHash,
    uniswap_factory: &TestContract,
    wcspr: &TestContract,
    uniswap_library: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "uniswap-v2-router.wasm",
        "uniswap-v2-router",
        owner,
        runtime_args! {
            "factory" => Key::Hash(uniswap_factory.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "library" => Key::Hash(uniswap_library.package_hash())
        },
        0,
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

    let wcspr = deploy_wcspr(&env, owner);
    let uniswap_library = deploy_uniswap_library(&env, owner);
    let uniswap_factory = deploy_uniswap_factory(&env, owner);
    let uniswap_router =
        deploy_uniswap_router(&env, owner, &uniswap_factory, &wcspr, &uniswap_library);
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &uniswap_factory);
    let uniswap_pair: TestContract =
        deploy_uniswap_pair(&env, owner, &flash_swapper, &uniswap_factory);

    let synthetic_token = SYNTHETICTOKENInstance::new(
        &env,
        "synthetic_token",
        owner,
        Key::Hash(wcspr.package_hash()),
        Key::Hash(uniswap_pair.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
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
fn test_get_trading_fee_amount() {
    let (_, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();

    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(wcspr.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );
    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(synthetic_token.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );

    let instance = SYNTHETICTOKENInstance::instance(synthetic_token);
    let previous_evaluation: U256 = 10.into();
    let current_evaluation: U256 = 10.into();
    instance.get_trading_fee_amount(owner, previous_evaluation, current_evaluation);
}

#[test]
fn test_get_amount_payout() {
    let (env, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();

    let instance = SYNTHETICTOKENInstance::instance(synthetic_token.clone());
    let amount: U256 = 10.into();
    let _: TestContract = deploy_fund_contract_purse_proxy(
        &env,
        env.next_user(),
        Key::Hash(synthetic_token.package_hash()),
        "fund_contract",
        U512::from(10000 as u128),
    );
    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(synthetic_token.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );
    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(wcspr.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );
    instance.get_amount_payout(owner, amount);
}

#[test]
fn test_get_wrapped_balance() {
    let (env, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token.clone());
    instance.get_wrapped_balance(owner);
}

#[test]
fn test_get_synthetic_balance() {
    let (env, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token.clone());
    instance.get_synthetic_balance(owner);
}

#[test]
fn test_get_evaluation() {
    let (env, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token.clone());
    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(synthetic_token.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );
    instance.get_evaluation(owner);
}

#[test]
fn test_get_pair_balances() {
    let (env, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token.clone());
    instance.get_pair_balances(owner);
}

#[test]
fn test_get_lp_token_balance() {
    let (env, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token.clone());
    instance.get_lp_token_balance(owner);
}

#[test]
fn test_get_liquidity_percent() {
    let (env, owner, synthetic_token, wcspr, _, _, _, _, uniswap_pair) = deploy();
    let instance = SYNTHETICTOKENInstance::instance(synthetic_token.clone());
    uniswap_pair.call_contract(
        owner,
        "erc20_mint",
        runtime_args! {
            "to" => Key::Hash(synthetic_token.package_hash()),
            "amount" => U256::from(1)
        },
        0,
    );
    instance.get_liquidity_percent(owner);
}
