use crate::liquidity_transformer_instance::LIQUIDITYTRANSFORMERInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256, U512};
use casperlabs_test_env::{TestContract, TestEnv};
use num_traits::cast::AsPrimitive;
use std::time::{SystemTime, UNIX_EPOCH};

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

const SCSPR_AMOUNT: U512 = U512([50_000_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
const TRANSFORMER_AMOUNT: U512 = U512([50_000_000, 0, 0, 0, 0, 0, 0, 0]);
const TWOTHOUSEND_CSPR: U512 = U512([2_000_000, 0, 0, 0, 0, 0, 0, 0]);

pub fn deploy_reserve_wise_purse_proxy(
    env: &TestEnv,
    sender: AccountHash,
    destination_package_hash: Key,
    destination_entrypoint: &str,
    investment_mode: u8,
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
            "investment_mode" => investment_mode,
            "amount" => amount,
        },
        time,
    )
}

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

pub fn deploy_deposit_purse_proxy(
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

pub fn deploy_set_liquidity_transfomer_purse_proxy(
    env: &TestEnv,
    sender: AccountHash,
    destination_package_hash: Key,
    destination_entrypoint: &str,
    immutable_transformer: Key,
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
            "immutable_transformer" => immutable_transformer,
            "amount" => amount,
        },
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
        env,
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

fn deploy_uniswap_factory(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => Key::from(owner)
        },
        0,
    )
}

fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    contract_name: &str,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
) -> TestContract {
    let flash_swapper_package_hash = flash_swapper.package_hash();
    TestContract::new(
        env,
        "pair-token.wasm",
        contract_name,
        owner,
        runtime_args! {
            "name" => "pair",
            "symbol" => "PAIR",
            "decimals" => 18_u8,
            "initial_supply" => U256::from(0),
            "callee_package_hash" => Key::Hash(flash_swapper_package_hash),
            "factory_hash" => Key::Hash(uniswap_factory.package_hash()),
        },
        0,
    )
}

fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
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
        0,
    )
}

fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        0,
    )
}

fn deploy_wcspr(env: &TestEnv, owner: AccountHash) -> TestContract {
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
        env,
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

fn deploy_transfer_helper(
    env: &TestEnv,
    owner: AccountHash,
    transfer_invoker: Key,
) -> TestContract {
    TestContract::new(
        env,
        "transfer_helper.wasm",
        "transfer_helper",
        owner,
        runtime_args! {
            "transfer_invoker" => transfer_invoker
        },
        0,
    )
}

fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "liquidity_guard.wasm",
        "liquidity_guard",
        owner,
        runtime_args! {},
        0,
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
        0,
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
            "launch_time" => U256::from(0),
        },
        0,
    )
}

#[allow(clippy::type_complexity)]
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
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let _wcspr = deploy_wcspr(&env, owner);
    let wcspr = deploy_wcspr(&env, owner);
    let uniswap_library = deploy_uniswap_library(&env, owner);
    let uniswap_factory = deploy_uniswap_factory(&env, owner);
    let uniswap_router =
        deploy_uniswap_router(&env, owner, &uniswap_factory, &wcspr, &uniswap_library);
    let erc20 = deploy_erc20(&env, owner);
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &uniswap_factory);
    let uniswap_pair: TestContract =
        deploy_uniswap_pair(&env, owner, "pair-1", &flash_swapper, &uniswap_factory);
    let uniswap_pair_wise: TestContract =
        deploy_uniswap_pair(&env, owner, "pair-2", &flash_swapper, &uniswap_factory);
    let liquidity_guard = deploy_liquidity_guard(&env, owner);
    let _erc20: Key = Key::Hash(erc20.package_hash());
    let scspr = deploy_scspr(
        &env,
        owner,
        &wcspr,
        &uniswap_pair,
        &uniswap_router,
        &uniswap_factory,
        SCSPR_AMOUNT,
    );
    let transfer_helper = deploy_transfer_helper(&env, owner, Key::Hash(scspr.package_hash()));
    let wise_token = deploy_wise(
        &env,
        owner,
        &scspr,
        &uniswap_router,
        &uniswap_factory,
        &uniswap_pair_wise,
        &liquidity_guard,
        &_wcspr,
        &erc20,
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
        TRANSFORMER_AMOUNT,
    );
    let proxy = LIQUIDITYTRANSFORMERInstance::proxy(
        &env,
        "proxy",
        owner,
        Key::Hash(liquidity_transformer.package_hash()),
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
        transfer_helper,
        uniswap_pair_wise,
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
    let erc20_key = Key::Hash(erc20.package_hash());
    let wcspr_key: Key = Key::Hash(wcspr.package_hash());

    proxy.call_contract(
        owner,
        "approve",
        runtime_args! {
            "token_address" => erc20_key,
            "spender" => package_liquidity,
            "amount" =>  U256::from(AMOUNT)
        },
        0,
    );

    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Account(owner)
        },
        0,
    );

    uniswap_factory.call_contract(
        owner,
        "create_pair",
        runtime_args! {
            "token_a" => erc20_key,
            "token_b" => wcspr_key,
            "pair_hash" => Key::Hash(uniswap_pair.package_hash())
        },
        0,
    );

    let router_package_hash = uniswap_router.package_hash();
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(router_package_hash)
        },
        0,
    );

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => package,
            "amount" => U256::from(AMOUNT)
        },
        0,
    );

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => package_liquidity,
            "amount" => U256::from(AMOUNT)
        },
        0,
    );

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => U256::from(AMOUNT)
        },
        0,
    );

    let _: TestContract = deploy_deposit_purse_proxy(
        env,
        owner,
        Key::Hash(wcspr.package_hash()),
        "deposit_no_return",
        U512::from(100_000_000_000_000_u128),
    );

    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_package_hash),
            "amount" => U256::from(AMOUNT)
        },
        0,
    );

    wcspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_package_hash),
            "amount" => U512::from(498_500_000_000_000_u128)
        },
        0,
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
        0,
    );
}

#[allow(clippy::too_many_arguments)]
fn reserve_wise(
    env: &TestEnv,
    lt: &TestContract,
    owner: AccountHash,
    uniswap_router: TestContract,
    uniswap_pair: TestContract,
    token: TestContract,
    scspr: TestContract,
    uniswap_factory: TestContract,
    helper: TestContract,
    time: u64,
) {
    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args! {
            "wise" => Key::Hash(token.package_hash())
        },
        0,
    );
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(scspr.package_hash())
        },
        0,
    );
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(uniswap_router.package_hash())
        },
        0,
    );
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(token.package_hash())
        },
        0,
    );
    scspr.call_contract(
        owner,
        "define_token",
        runtime_args! {
            "wise_token" => Key::Hash(token.package_hash()),
        },
        0,
    );
    assert!(
        scspr.query_named_key::<bool>("token_defined".into()),
        "Token not defined"
    );
    scspr.call_contract(
        owner,
        "define_helper",
        runtime_args! {
            "transfer_helper" => Key::Hash(helper.package_hash()),
        },
        0,
    );
    assert!(
        scspr.query_named_key::<bool>("helper_defined".into()),
        "Helper not defined"
    );

    token.call_contract(owner, "create_pair", runtime_args! {}, 0);
    scspr.call_contract(
        owner,
        "create_pair",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair.package_hash()),
        },
        0,
    );

    // Using session code as caller of purse is required for reserving wise
    TestContract::new(
        env,
        "purse-proxy.wasm",
        "set_liquidity_transfomer_call",
        owner,
        runtime_args! {
            "destination_entrypoint" => "set_liquidity_transfomer",
            "destination_package_hash" => Key::Hash(token.package_hash()),
            "immutable_transformer" => Key::Hash(lt.package_hash()),
            "amount" => U512::from(0)
        },
        time,
    );
    // NOW CALLS TIME SHOULD BE IN ADVANCED 'TIME'
    // Using session code as caller of purse is required for reserving wise

    TestContract::new(
        env,
        "purse-proxy.wasm",
        "reserve_wise_call",
        owner,
        runtime_args! {
            "destination_entrypoint" => "reserve_wise",
            "destination_package_hash" => Key::Hash(lt.package_hash()),
            "investment_mode" => 1_u8,
            "amount" => TWOTHOUSEND_CSPR
        },
        time,
    );
}

#[allow(clippy::too_many_arguments)]
fn forward_liquidity(
    env: &TestEnv,
    lt: &TestContract,
    owner: AccountHash,
    uniswap_router: TestContract,
    uniswap_pair: TestContract,
    token: TestContract,
    scspr: TestContract,
    uniswap_factory: TestContract,
    helper: TestContract,
    uniswap_pair_wise: TestContract,
) {
    const TIME: u64 = 300_000_000;

    reserve_wise(
        env,
        lt,
        owner,
        uniswap_router,
        uniswap_pair,
        token,
        scspr,
        uniswap_factory,
        helper,
        TIME,
    );

    lt.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair_wise.package_hash())
        },
        TIME * 150_000,
    );
}

#[test]
fn test_deploy() {
    let (_, _, _, _, _, _, _, _, _, _, _, _, _) = deploy();
}

#[test]
fn test_current_wise_day() {
    let (_, _, owner, proxy, _, _, _, _, _, _, _, _, _) = deploy();

    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);

    const DAYS: u64 = 33;
    const TIME: u64 = DAYS * 86400 * 1000;

    proxy.current_stakeable_day(owner, TIME);

    let ret: u64 = proxy.result();
    assert_eq!(ret, DAYS);
}

#[test]
fn test_set_settings() {
    let (_, liquidity_contract, owner, _, _, _, _, pair, wise, scspr, _, _, _) = deploy();

    liquidity_contract.call_contract(
        owner,
        "set_settings",
        runtime_args! {
            "wise_token" =>  Key::Hash(wise.package_hash()),
            "uniswap_pair" => Key::Hash(pair.package_hash()),
            "synthetic_cspr" => Key::Hash(scspr.package_hash())
        },
        0,
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
    let (_, liquidity_contract, owner, _, _, _, _, _, _, _, _, _, _) = deploy();

    let res: Key = liquidity_contract.query_named_key("settings_keeper".to_string());
    let zero: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(res, zero);

    liquidity_contract.call_contract(owner, "renounce_keeper", runtime_args! {}, 0);

    let res: Key = liquidity_contract.query_named_key("settings_keeper".to_string());
    assert_eq!(res, zero);
}

#[test]
fn test_reserve_wise() {
    let (env, liquidity_contract, owner, _, _, _, _, _, _, _, _, _, _) = deploy();

    let liquidity: Key = Key::Hash(liquidity_contract.package_hash());

    let investment_mode: u8 = 1;
    let msg_value: U512 = 75757576.into();

    const DAYS: u64 = 15;
    const TIME: u64 = DAYS * 86400 * 1000;

    let _: TestContract = deploy_reserve_wise_purse_proxy(
        &env,
        owner,
        liquidity,
        "reserve_wise",
        investment_mode,
        msg_value,
        TIME,
    );
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
        TIME,
    );
}

#[test]
fn test_forward_liquidity() {
    let (
        env,
        liquidity_contract,
        owner,
        _,
        _,
        _,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        uniswap_pair_wise,
    ) = deploy();

    forward_liquidity(
        &env,
        &liquidity_contract,
        owner,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        uniswap_pair_wise,
    );
}

#[test]
fn test_payout_investor_address() {
    let (
        env,
        liquidity_contract,
        owner,
        proxy,
        _,
        _,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        uniswap_pair_wise,
    ) = deploy();

    forward_liquidity(
        &env,
        &liquidity_contract,
        owner,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        uniswap_pair_wise,
    );

    proxy.call_contract(
        owner,
        "payout_investor_address",
        runtime_args! {
            "investor_address" => Key::Account(owner)
        },
        0,
    );

    let ret: U256 = proxy.query_named_key("result".to_string());
    assert_eq!(ret, 2000000000.into());
}

#[test]
fn test_get_my_tokens() {
    let (
        env,
        liquidity_contract,
        owner,
        _,
        _,
        _,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        uniswap_pair_wise,
    ) = deploy();

    forward_liquidity(
        &env,
        &liquidity_contract,
        owner,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        uniswap_pair_wise,
    );

    liquidity_contract.call_contract(owner, "get_my_tokens", runtime_args! {}, 0);
}

#[test]
fn test_prepare_path() {
    let (_, _, owner, proxy, erc20, wcspr, _, _, _, _, _, _, _) = deploy();

    let proxy = LIQUIDITYTRANSFORMERInstance::instance(proxy);

    let token_address: Key = Key::Hash(erc20.package_hash());

    proxy.prepare_path(owner, token_address);
    let ret: Vec<Key> = proxy.result();
    assert_eq!(ret[0], Key::Hash(erc20.package_hash()));
    assert_eq!(ret[1], Key::Hash(wcspr.package_hash()));
}

#[test]
fn test_request_refund() {
    let (
        env,
        liquidity_contract,
        owner,
        _,
        _,
        _,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        _,
    ) = deploy();

    const _DAYS: u64 = 10;
    const _TIME: u64 = _DAYS * 86400 * 1000;

    reserve_wise(
        &env,
        &liquidity_contract,
        owner,
        uniswap_router,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        helper,
        _TIME,
    );

    // TIME PASSED, NOW CAN REFUND

    const DAYS: u64 = 30;
    const TIME: u64 = DAYS * 86400 * 1000;

    TestContract::new(
        &env,
        "purse-proxy.wasm",
        "request_refund_call",
        owner,
        runtime_args! {
            "destination_entrypoint" => "request_refund",
            "destination_package_hash" => Key::Hash(liquidity_contract.package_hash()),
            "amount" => U512::from(0)
        },
        TIME,
    );
    let ret: (U256, U256) = env.query_account_named_key(owner, &["result".into()]);

    let token_cost: U256 = 2000000000.into();

    assert_eq!(
        ret,
        (
            <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(TWOTHOUSEND_CSPR),
            token_cost
        ),
        "Invalid refund"
    );
}
