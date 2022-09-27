use std::time::{SystemTime, UNIX_EPOCH};

use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::liquidity_transformer_instance::{now, LIQUIDITYTRANSFORMERInstance};

const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;

//
// --- NOTE FOR HANDLING TIME ---
//
// Initail deployments are done with 0 time
//
// Time can be manipulated while calling functions as follow
//
// 1 day in seconds = 86400;
// 1 day in millisecond = 86400 * 1000
//
// EXAMPLE
// If needed 15 days => 15 * 86400 * 1000 (Time required in 'ms')
//

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

pub fn deploy_deposit_purse_proxy(
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

pub fn deploy_set_liquidity_transfomer_purse_proxy(
    env: &TestEnv,
    sender: AccountHash,
    destination_package_hash: Key,
    destination_entrypoint: &str,
    immutable_transformer: Key,
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
            "immutable_transformer" => immutable_transformer,
            "amount" => amount,
        },
        time,
    )
}

pub fn deploy_forward_liquidity_purse_proxy(
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
            "amount" => amount
        },
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

fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
    time: u64,
) -> TestContract {
    let flash_swapper_package_hash = flash_swapper.package_hash();
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
            "callee_package_hash" => Key::Hash(flash_swapper_package_hash),
            "factory_hash" => Key::Hash(uniswap_factory.package_hash()),
        },
        time,
    )
}

fn deploy_erc20(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 18_u8,
            "initial_supply" => U256::from(404000000000000000_u128)
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

fn deploy_transfer_helper(
    env: &TestEnv,
    owner: AccountHash,
    transfer_invoker: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "transfer_helper.wasm",
        "transfer_helper",
        owner,
        runtime_args! {
            "transfer_invoker" => transfer_invoker
        },
        time,
    )
}

fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "liquidity_guard.wasm",
        "liquidity_guard",
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
    synthetic_token: &TestContract,
    transfer_helper: &TestContract,
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
            "synthetic_token" => Key::Hash(synthetic_token.package_hash()),
            "transfer_helper" => Key::Hash(transfer_helper.package_hash()),
        },
        time,
    )
}

fn deploy_synthetic_token(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    transfer_helper: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "synthetic_token.wasm",
        "synthetic_token",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.package_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.package_hash()),
            "transfer_helper" => Key::Hash(transfer_helper.package_hash()),
        },
        time,
    )
}

#[allow(clippy::too_many_arguments)]
fn deploy_wise(
    env: &TestEnv,
    owner: AccountHash,
    scspr: &TestContract,
    router: &TestContract,
    factory: &TestContract,
    pair: &TestContract,
    liquidity_guard: &TestContract,
    _wcspr: &TestContract,
    erc20: &TestContract,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "stakeabletoken.wasm",
        "stakeabletoken",
        owner,
        runtime_args! {
            "scspr" => Key::Hash(scspr.package_hash()),
            "router" => Key::Hash(router.package_hash()),
            "factory" => Key::Hash(factory.package_hash()),
            "pair" => Key::Hash(pair.package_hash()),
            "liquidity_guard" => Key::Hash(liquidity_guard.package_hash()),
            "wcspr" => Key::Hash(_wcspr.package_hash()),
            "erc20" => Key::Hash(erc20.package_hash()),
            "launch_time" => U256::from(now()),
        },
        time,
    )
}

fn deploy() -> (
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
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let _wcspr = deploy_wcspr(&env, owner, now());
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
    let erc20 = deploy_erc20(&env, owner, now());
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &uniswap_factory, now());
    let uniswap_pair: TestContract =
        deploy_uniswap_pair(&env, owner, &flash_swapper, &uniswap_factory, now());
    let liquidity_guard = deploy_liquidity_guard(&env, owner, now());
    let transfer_helper = deploy_transfer_helper(&env, owner, Key::Account(owner), now());

    let _erc20: Key = Key::Hash(erc20.package_hash());

    let synthetic_token = deploy_synthetic_token(
        &env,
        owner,
        &_wcspr,
        &uniswap_pair,
        &uniswap_router,
        &transfer_helper,
        now(),
    );
    deploy_transfer_helper(
        &env,
        owner,
        Key::Hash(synthetic_token.package_hash()),
        now(),
    );
    let scspr = deploy_scspr(
        &env,
        owner,
        &wcspr,
        &uniswap_pair,
        &uniswap_router,
        &uniswap_factory,
        &synthetic_token,
        &transfer_helper,
        now(),
    );
    let wise_token = deploy_wise(
        &env,
        owner,
        &scspr,
        &uniswap_router,
        &uniswap_factory,
        &uniswap_pair,
        &liquidity_guard,
        &_wcspr,
        &erc20,
        now(),
    );

    let liquidity_transformer = LIQUIDITYTRANSFORMERInstance::new(
        &env,
        "LIQUIDITY_TRANSFORMER",
        owner,
        Key::Hash(wise_token.package_hash()),
        Key::Hash(scspr.package_hash()),
        Key::Hash(uniswap_pair.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(wcspr.package_hash()),
        now(),
    );

    let proxy = LIQUIDITYTRANSFORMERInstance::proxy(
        &env,
        "proxy",
        owner,
        Key::Hash(liquidity_transformer.package_hash()),
        now(),
    );

    (
        env,
        liquidity_transformer,
        owner,
        proxy,
        erc20,
        wcspr,
        uniswap_router,
        uniswap_pair,
        wise_token,
        scspr,
        uniswap_factory,
        synthetic_token,
    )
}

#[allow(clippy::too_many_arguments)]
fn add_liquidity(
    env: &TestEnv,
    liquidity_contract: TestContract,
    owner: AccountHash,
    proxy: &TestContract,
    erc20: &TestContract,
    uniswap_router: TestContract,
    uniswap_pair: TestContract,
    wcspr: TestContract,
    uniswap_factory: TestContract,
) {
    const AMOUNT: u128 = 100_000_000_000_000_000;
    let package: Key = Key::Hash(proxy.package_hash());
    let package_liquidity: Key = Key::Hash(liquidity_contract.package_hash());
    let proxy_inst = LIQUIDITYTRANSFORMERInstance::instance(proxy.clone());
    let erc20_key = Key::Hash(erc20.package_hash());
    let wcspr_key: Key = Key::Hash(wcspr.package_hash());

    proxy_inst.approve(
        owner,
        erc20_key,
        package_liquidity,
        U256::from(AMOUNT),
        now(),
    );

    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Account(owner)
        },
        now(),
    );

    uniswap_factory.call_contract(
        owner,
        "create_pair",
        runtime_args! {
            "token_a" => erc20_key,
            "token_b" => wcspr_key,
            "pair_hash" => Key::Hash(uniswap_pair.package_hash())
        },
        now(),
    );

    let router_package_hash = uniswap_router.package_hash();
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(router_package_hash)
        },
        now(),
    );

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => package,
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => package_liquidity,
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );

    let _: TestContract = deploy_deposit_purse_proxy(
        env,
        owner,
        Key::Hash(wcspr.package_hash()),
        "deposit_no_return",
        U512::from(100_000_000_000_000_u128),
        now(),
    );

    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_package_hash),
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );

    wcspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_package_hash),
            "amount" => U512::from(498_500_000_000_000_u128)
        },
        now(),
    );

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap_router.call_contract(
        owner,
        "add_liquidity_js_client",
        runtime_args! {
            "deadline" => U256::from(deadline),
            "token_a" => erc20_key,
            "token_b" => wcspr_key,
            "amount_a_desired" => U256::from(1_000_000_000_000_u128),
            "amount_b_desired" => U256::from(1_000_000_000_000_u128),
            "amount_a_min" => U256::from(100_000_000_000_u128),
            "amount_b_min" => U256::from(100_000_000_000_u128),
            "to" => Key::Hash(uniswap_pair.package_hash()),
            "pair" => Some(Key::Hash(uniswap_pair.package_hash())),
        },
        now(),
    );
}

#[allow(clippy::too_many_arguments)]
fn forward_liquidity(
    env: &TestEnv,
    liquidity_contract: TestContract,
    owner: AccountHash,
    proxy: TestContract,
    erc20: TestContract,
    uniswap_router: TestContract,
    uniswap_pair: TestContract,
    wise: TestContract,
    scspr: TestContract,
    uniswap_factory: TestContract,
    wcspr: TestContract,
) {
    let uniswap_pair_package = uniswap_pair.package_hash();

    const MINTED: u128 = 45;

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(uniswap_pair_package),
            "amount" => U256::from(MINTED)
        },
        now(),
    );

    let uniswap_router_package = uniswap_router.package_hash();
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(uniswap_router_package)
        },
        now(),
    );

    let _: TestContract = deploy_fund_contract_purse_proxy(
        env,
        env.next_user(),
        Key::Hash(scspr.package_hash()),
        "fund_contract",
        U512::from(10000),
        now(),
    );

    let liquidity_package = liquidity_contract.package_hash();
    let _: TestContract = deploy_set_liquidity_transfomer_purse_proxy(
        env,
        owner,
        Key::Hash(wise.package_hash()),
        "set_liquidity_transfomer",
        Key::Hash(liquidity_package),
        0.into(),
        now(),
    );

    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args! {
            "wise" => Key::Hash(wise.package_hash())
        },
        now(),
    );

    let scspr_package = scspr.package_hash();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "amount" => U256::from(MINTED),
            "to" => Key::Hash(scspr_package)
        },
        now(),
    );

    let uniswap_router_package = uniswap_router.package_hash();
    scspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "amount" => U256::from(MINTED),
            "spender" => Key::Hash(uniswap_router_package)
        },
        now(),
    );

    const AMOUNT: u128 = 100_000_000_000_000_000;
    let scspr_package: ContractPackageHash = scspr.query_named_key("self_package_hash".to_string());
    scspr.call_contract(
        owner,
        "mint",
        runtime_args! {
            "recipient" => Key::from(scspr_package),
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );

    scspr.call_contract(
        owner,
        "mint",
        runtime_args! {
            "recipient" => Key::Hash(scspr.package_hash()),
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );

    let _: TestContract = deploy_deposit_purse_proxy(
        env,
        owner,
        Key::Hash(wcspr.package_hash()),
        "deposit_no_return",
        U512::from(100000),
        now(),
    );

    wise.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(liquidity_package),
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );

    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);
    let liquidity: Key = Key::Hash(liquidity_contract.package_hash());

    let investment_mode: u8 = 1;
    let msg_value: U256 = (75757576000000000_u128).into();

    const _DAYS: u64 = 15;
    const _TIME: u64 = _DAYS * 86400 * 1000;

    proxy.reserve_wise(owner, liquidity, investment_mode, msg_value, now() + _TIME);

    const __DAYS: u64 = 25;
    const __TIME: u64 = __DAYS * 86400 * 1000;

    let _: TestContract = deploy_forward_liquidity_purse_proxy(
        env,
        owner,
        liquidity,
        "forward_liquidity",
        U512::from("1000000000"),
        now() + __TIME,
    );
}

#[test]
fn test_deploy() {
    let (_, _, _, _, _, _, _, _, _, _, _, _) = deploy();
}

#[test]
fn test_current_stakeable_day() {
    let (_, _, owner, proxy, _, _, _, _, _, _, _, _) = deploy();
    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);
    const DAYS: u64 = 10;
    proxy.current_stakeable_day(owner, now() + (DAYS * MILLI_SECONDS_IN_DAY));
    let ret: u64 = proxy.result();
    assert_eq!(ret, DAYS);
}

#[test]
fn test_set_settings() {
    let (_, liquidity_contract, owner, _, _, _, _, pair, wise, scspr, _, _) = deploy();

    liquidity_contract.call_contract(
        owner,
        "set_settings",
        runtime_args! {
            "wise_token" =>  Key::Hash(wise.package_hash()),
            "uniswap_pair" => Key::Hash(pair.package_hash()),
            "synthetic_cspr" => Key::Hash(scspr.package_hash())
        },
        now(),
    );

    let setted_wise_contract: Key = liquidity_contract.query_named_key("wise_contract".to_string());
    let setted_uniswap_pair: Key = liquidity_contract.query_named_key("uniswap_pair".to_string());
    let setted_scspr: Key = liquidity_contract.query_named_key("scspr".to_string());

    assert_eq!(setted_wise_contract, Key::Hash(wise.package_hash()));
    assert_eq!(setted_uniswap_pair, Key::Hash(pair.package_hash()));
    assert_eq!(setted_scspr, Key::Hash(scspr.package_hash()));
}

#[test]
fn test_renounce_keeper() {
    let (_, liquidity_contract, owner, _, _, _, _, _, _, _, _, _) = deploy();

    let res: Key = liquidity_contract.query_named_key("settings_keeper".to_string());
    let zero: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(res, zero);

    liquidity_contract.call_contract(owner, "renounce_keeper", runtime_args! {}, now());

    let res: Key = liquidity_contract.query_named_key("settings_keeper".to_string());
    assert_eq!(res, zero);
}

#[test]
fn test_reserve_wise() {
    let (_, liquidity_contract, owner, proxy, _, _, _, _, _, _, _, _) = deploy();

    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);
    let liquidity: Key = Key::Hash(liquidity_contract.package_hash());

    let investment_mode: u8 = 1;
    let msg_value: U256 = (75757576000000000_u128).into();

    const DAYS: u64 = 15;
    const TIME: u64 = DAYS * 86400 * 1000;

    proxy.reserve_wise(owner, liquidity, investment_mode, msg_value, now() + TIME);
}

#[test]
fn test_reserve_wise_with_token() {
    let (
        env,
        liquidity_contract,
        owner,
        proxy,
        erc20,
        wcspr,
        uniswap_router,
        uniswap_pair,
        _,
        _,
        uniswap_factory,
        _,
    ) = deploy();

    add_liquidity(
        &env,
        liquidity_contract,
        owner,
        &proxy,
        &erc20,
        uniswap_router,
        uniswap_pair,
        wcspr,
        uniswap_factory,
    );

    let proxy_key: Key = Key::Hash(proxy.package_hash());
    let investment_mode: u8 = 1;
    let proxy_inst = LIQUIDITYTRANSFORMERInstance::instance(proxy);

    const DAYS: u64 = 15;
    const TIME: u64 = DAYS * 86400 * 1000;

    const AMOUNT: u128 = 100_000_000_000_000_000;

    proxy_inst.reserve_wise_with_token(
        owner,
        proxy_key,
        Key::Hash(erc20.package_hash()),
        U256::from(AMOUNT),
        investment_mode,
        now() + TIME,
    );
}

#[test]
fn test_forward_liquidity() {
    let (
        env,
        liquidity_contract,
        owner,
        proxy,
        erc20,
        wcspr,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        _,
    ) = deploy();

    forward_liquidity(
        &env,
        liquidity_contract,
        owner,
        proxy,
        erc20,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        wcspr,
    );
}

#[test]
fn test_payout_investor_address() {
    let (
        env,
        liquidity_contract,
        owner,
        proxy,
        erc20,
        wcspr,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        _,
    ) = deploy();

    forward_liquidity(
        &env,
        liquidity_contract,
        owner,
        proxy.clone(),
        erc20,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        wcspr,
    );

    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);

    proxy.payout_investor_address(owner, Key::Account(owner), now());

    let ret: U256 = proxy.result();
    assert_eq!(ret, (264000000000000000_u128).into());
}

#[test]
fn test_get_my_tokens() {
    let (
        env,
        liquidity_contract,
        owner,
        proxy,
        erc20,
        wcspr,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        _,
    ) = deploy();

    forward_liquidity(
        &env,
        liquidity_contract.clone(),
        owner,
        proxy,
        erc20,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        wcspr,
    );

    let liquidity_transformer = LIQUIDITYTRANSFORMERInstance::instance(liquidity_contract);

    liquidity_transformer.get_my_tokens(owner, now());
}

#[test]
fn test_prepare_path() {
    let (_, _, owner, proxy, erc20, wcspr, _, _, _, _, _, _) = deploy();

    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);

    let token_address: Key = Key::Hash(erc20.package_hash());

    proxy.prepare_path(owner, token_address, now());
    let ret: Vec<Key> = proxy.result();
    assert_eq!(ret[0], Key::Hash(erc20.package_hash()));
    assert_eq!(ret[1], Key::Hash(wcspr.package_hash()));
}

#[test]
fn test_request_refund() {
    let (_, liquidity_contract, owner, proxy, _, _, _, _, _, _, _, _) = deploy();

    let proxy_key: Key = Key::Hash(proxy.package_hash());
    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);
    let liquidity: Key = Key::Hash(liquidity_contract.package_hash());

    let investment_mode: u8 = 1;
    let msg_value: U256 = (75757576000000000_u128).into();

    const _DAYS: u64 = 15;
    const _TIME: u64 = _DAYS * 86400 * 1000;

    proxy.reserve_wise(owner, liquidity, investment_mode, msg_value, now() + _TIME);

    // TIME PASSED, NOW CAN REFUND

    const DAYS: u64 = 30;
    const TIME: u64 = DAYS * 86400 * 1000;

    proxy.request_refund(owner, liquidity, proxy_key, now() + TIME);

    let token_cost: U256 = U256::from(264000000000000000_u128);
    let ret: (U256, U256) = proxy.result();
    assert_eq!(ret, (msg_value, token_cost));
}
