use crate::liquidity_transformer_instance::LIQUIDITYTRANSFORMERInstance;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256, U512,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::time::SystemTime;

const SCSPR_AMOUNT: U512 = U512([50_000_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
const TRANSFORMER_AMOUNT: U512 = U512([50_000_000, 0, 0, 0, 0, 0, 0, 0]);
const TWOTHOUSEND_CSPR: U512 = U512([2_000_000_000_000, 0, 0, 0, 0, 0, 0, 0]);

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
    contract_name: &str,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
    time: u64,
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
            "decimals" => 9_u8,
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
        "erc20",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 18_u8,
            "initial_supply" => U256::from(0)
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

fn deploy_wcspr(
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

#[allow(clippy::too_many_arguments)]
fn deploy_wise(
    env: &TestEnv,
    owner: AccountHash,
    scspr: &TestContract,
    router: &TestContract,
    factory: &TestContract,
    pair: &TestContract,
    liquidity_guard: &TestContract,
    wcspr: &TestContract,
    erc20: &TestContract,
    launch_time: U256,
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
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "erc20" => Key::Hash(erc20.package_hash()),
            "launch_time" => launch_time
        },
        time,
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
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let wcspr = deploy_wcspr(
        &env,
        owner,
        "Wrapped CSPR".into(),
        "WCSPR".into(),
        18,
        now(),
    );
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
    let pair_scspr: TestContract = deploy_uniswap_pair(
        &env,
        owner,
        "pair-1",
        &flash_swapper,
        &uniswap_factory,
        now(),
    );
    let pair_wise: TestContract = deploy_uniswap_pair(
        &env,
        owner,
        "pair-2",
        &flash_swapper,
        &uniswap_factory,
        now(),
    );
    let liquidity_guard = deploy_liquidity_guard(&env, owner, now());
    let scspr = deploy_scspr(
        &env,
        owner,
        &wcspr,
        &pair_scspr,
        &uniswap_router,
        &uniswap_factory,
        SCSPR_AMOUNT,
        now(),
    );
    let wise_token = deploy_wise(
        &env,
        owner,
        &scspr,
        &uniswap_router,
        &uniswap_factory,
        &pair_wise,
        &liquidity_guard,
        &wcspr,
        &erc20,
        (now() - 172800000).into(), // 172800000 == 2 days in ms (launch time set in past for testing)
        now(),
    );
    let liquidity_transformer = LIQUIDITYTRANSFORMERInstance::new(
        &env,
        "LIQUIDITY_TRANSFORMER",
        owner,
        Key::Hash(wise_token.package_hash()),
        Key::Hash(scspr.package_hash()),
        Key::Hash(pair_wise.package_hash()),
        Key::Hash(pair_scspr.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(wcspr.package_hash()),
        TRANSFORMER_AMOUNT,
        now(),
    );
    (
        env,
        liquidity_transformer,
        owner,
        pair_scspr,
        wise_token,
        scspr,
        uniswap_factory,
        pair_wise,
    )
}

#[allow(clippy::too_many_arguments)]
fn forward_liquidity(
    env: &TestEnv,
    lt: &TestContract,
    owner: AccountHash,
    uniswap_pair: TestContract,
    token: &TestContract,
    scspr: &TestContract,
    uniswap_factory: TestContract,
    uniswap_pair_wise: TestContract,
) -> u64 {
    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args! {
            "wise" => Key::Hash(token.package_hash())
        },
        now(),
    );
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(scspr.package_hash())
        },
        now(),
    );
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(token.package_hash())
        },
        now(),
    );
    token.call_contract(owner, "create_pair", runtime_args! {}, now());
    scspr.call_contract(
        owner,
        "create_pair",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair.package_hash()),
        },
        now(),
    );
    // Using session code as transformer purse fetch with access is required
    session_code_call(
        env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(token.package_hash()),
            "entrypoint" => "set_liquidity_transfomer",
            "immutable_transformer" => Key::Hash(lt.package_hash()),
        },
        now(),
    );
    // Forward liquidity to be done after investment days
    const INVESTMENT_DAY: u64 = 20; // Some random day after investment days passed
    const INVESTMENT_DAY_TIME: u64 = INVESTMENT_DAY * 86400 * 1000;
    lt.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair_wise.package_hash())
        },
        now() + INVESTMENT_DAY_TIME,
    );
    now() + INVESTMENT_DAY_TIME
}

#[test]
fn test_reserve_claim_flow() {
    let (
        env,
        liquidity_transformer,
        owner,
        uniswap_pair,
        wise,
        scspr,
        uniswap_factory,
        uniswap_pair_wise,
    ) = deploy();

    let (user1, user2) = (env.next_user(), env.next_user());
    // Using session code as caller of purse is required for reserving wise (multiple reservations)
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "reserve_wise",
            "investment_mode" => 1_u8,
            "amount" => TWOTHOUSEND_CSPR
        },
        now(),
    );
    session_code_call(
        &env,
        user1,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "reserve_wise",
            "investment_mode" => 1_u8,
            "amount" => TWOTHOUSEND_CSPR + TWOTHOUSEND_CSPR
        },
        now(),
    );
    session_code_call(
        &env,
        user2,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "reserve_wise",
            "investment_mode" => 1_u8,
            "amount" => TWOTHOUSEND_CSPR * 3
        },
        now(),
    );

    let time = forward_liquidity(
        &env,
        &liquidity_transformer,
        owner,
        uniswap_pair,
        &wise,
        &scspr,
        uniswap_factory,
        uniswap_pair_wise,
    );

    // Balance check before
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(balance, 0.into(), "Already have some wise tokens");
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(user1)))
        .unwrap_or_default();
    assert_eq!(balance, 0.into(), "Already have some wise tokens");
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(user2)))
        .unwrap_or_default();
    assert_eq!(balance, 0.into(), "Already have some wise tokens");

    // Claiming tokens
    liquidity_transformer.call_contract(owner, "get_my_tokens", runtime_args! {}, time);
    liquidity_transformer.call_contract(user1, "get_my_tokens", runtime_args! {}, time);
    liquidity_transformer.call_contract(user2, "get_my_tokens", runtime_args! {}, time);

    // Balance check after
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(
        balance,
        2640002000000000u64.into(), // calculated amount in contract
        "Tokens not transfered to owner"
    );
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(user1)))
        .unwrap_or_default();
    assert_eq!(
        balance,
        5280005000000000u64.into(), // calculated amount in contract
        "Tokens not transfered to owner"
    );
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(user2)))
        .unwrap_or_default();
    assert_eq!(
        balance,
        7920007000000000u64.into(), // calculated amount in contract
        "Tokens not transfered to owner"
    );
}
