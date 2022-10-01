use crate::liquidity_transformer_instance::LIQUIDITYTRANSFORMERInstance;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256, U512,
};
use casperlabs_test_env::{TestContract, TestEnv};
use num_traits::cast::AsPrimitive;
use std::time::{SystemTime, UNIX_EPOCH};

const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
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

pub fn session_code_result<T: CLTyped + FromBytes>(env: &TestEnv, sender: AccountHash) -> T {
    env.query_account_named_key(sender, &["result".into()])
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
    let uniswap_pair: TestContract = deploy_uniswap_pair(
        &env,
        owner,
        "pair-1",
        &flash_swapper,
        &uniswap_factory,
        now(),
    );
    let uniswap_pair_wise: TestContract = deploy_uniswap_pair(
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
        &uniswap_pair,
        &uniswap_router,
        &uniswap_factory,
        SCSPR_AMOUNT,
        now(),
    );
    let transfer_helper =
        deploy_transfer_helper(&env, owner, Key::Hash(scspr.package_hash()), now());
    let wise_token = deploy_wise(
        &env,
        owner,
        &scspr,
        &uniswap_router,
        &uniswap_factory,
        &uniswap_pair_wise,
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
        Key::Hash(uniswap_pair.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(wcspr.package_hash()),
        TRANSFORMER_AMOUNT,
        now(),
    );
    (
        env,
        liquidity_transformer,
        owner,
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

fn add_liquidity(
    env: &TestEnv,
    owner: AccountHash,
    erc20: &TestContract,
    uniswap_router: TestContract,
    uniswap_pair: TestContract,
    wcspr: TestContract,
    uniswap_factory: TestContract,
) {
    const AMOUNT: u128 = 100_000_000_000_000_000;
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
            "token_a" => Key::Hash(erc20.package_hash()),
            "token_b" => Key::Hash(wcspr.package_hash()),
            "pair_hash" => Key::Hash(uniswap_pair.package_hash())
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
    session_code_call(
        env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(wcspr.package_hash()),
            "entrypoint" => "deposit_no_return",
            "amount" => U512::from(100_000_000_000_000_u128),
        },
        now(),
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(uniswap_router.package_hash()),
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );
    wcspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(uniswap_router.package_hash()),
            "amount" => U512::from(100_000_000_000_000_u128)
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
            "token_a" => Key::Hash(erc20.package_hash()),
            "token_b" => Key::Hash(wcspr.package_hash()),
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
fn initialize_flow(
    env: &TestEnv,
    lt: &TestContract,
    owner: AccountHash,
    uniswap_router: TestContract,
    uniswap_pair: TestContract,
    token: &TestContract,
    scspr: &TestContract,
    uniswap_factory: TestContract,
    helper: TestContract,
) {
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
            "white_list" => Key::Hash(uniswap_router.package_hash())
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
    scspr.call_contract(
        owner,
        "define_token",
        runtime_args! {
            "wise_token" => Key::Hash(token.package_hash()),
        },
        now(),
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
        now(),
    );
    assert!(
        scspr.query_named_key::<bool>("helper_defined".into()),
        "Helper not defined"
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
}

#[test]
fn test_deploy() {
    let (_, _, _, _, _, _, _, _, _, _, _, _) = deploy();
}

#[test]
fn test_current_stakeable_day() {
    let (env, lt, owner, _, _, _, _, _, _, _, _, _) = deploy();
    const DAYS: u64 = 10;
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(lt.package_hash()),
            "entrypoint" => "current_stakeable_day"
        },
        now() + (DAYS * MILLI_SECONDS_IN_DAY),
    );
    let ret: u64 = session_code_result(&env, owner);
    assert_eq!(ret - 2, DAYS, "Invalid stakeable day"); // - 2 for past launch time balance
}

#[test]
fn test_set_settings() {
    let (_, liquidity_transformer, owner, _, _, _, pair, wise, scspr, _, _, _) = deploy();
    liquidity_transformer.call_contract(
        owner,
        "set_settings",
        runtime_args! {
            "wise_token" =>  Key::Hash(wise.package_hash()),
            "uniswap_pair" => Key::Hash(pair.package_hash()),
            "synthetic_cspr" => Key::Hash(scspr.package_hash())
        },
        now(),
    );
    let setted_wise_contract: Key =
        liquidity_transformer.query_named_key("wise_contract".to_string());
    let setted_uniswap_pair: Key =
        liquidity_transformer.query_named_key("uniswap_pair".to_string());
    let setted_scspr: Key = liquidity_transformer.query_named_key("scspr".to_string());
    assert_eq!(
        setted_wise_contract,
        Key::Hash(wise.package_hash()),
        "wise address not set"
    );
    assert_eq!(
        setted_uniswap_pair,
        Key::Hash(pair.package_hash()),
        "uniswap pair address not set"
    );
    assert_eq!(
        setted_scspr,
        Key::Hash(scspr.package_hash()),
        "scspr address not set"
    );
}

#[test]
fn test_renounce_keeper() {
    let (_, liquidity_transformer, owner, _, _, _, _, _, _, _, _, _) = deploy();
    let res: Key = liquidity_transformer.query_named_key("settings_keeper".to_string());
    let zero: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(res, zero, "Keeper already zero address");
    liquidity_transformer.call_contract(owner, "renounce_keeper", runtime_args! {}, 0);
    let res: Key = liquidity_transformer.query_named_key("settings_keeper".to_string());
    assert_eq!(res, zero, "Keeper not renounced");
}

#[test]
fn test_reserve_wise() {
    let (env, liquidity_transformer, owner, _, _, _, _, _, _, _, _, _) = deploy();
    let investment_mode: u8 = 1;
    let msg_value: U512 = 75757576.into(); // this value because min value constraint (MIN = 75757575)
    let investor_balance: U256 = liquidity_transformer
        .query_dictionary("investor_balance", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(
        investor_balance,
        0.into(),
        "Investor has already some wise balance"
    );
    const DAYS: u64 = 12;
    const TIME: u64 = DAYS * 86400 * 1000;
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "reserve_wise",
            "investment_mode" => investment_mode,
            "amount" => msg_value,
        },
        now() + TIME,
    );
    let investor_balance: U256 = liquidity_transformer
        .query_dictionary("investor_balance", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(
        investor_balance,
        <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(msg_value),
        "Investor wise balance not increased"
    );
}

#[test]
fn test_reserve_wise_with_token() {
    let (
        env,
        liquidity_transformer,
        owner,
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
        owner,
        &erc20,
        uniswap_router,
        uniswap_pair,
        wcspr,
        uniswap_factory,
    );
    const AMOUNT: u128 = 100_000_000;
    let investment_mode: u8 = 1;
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(liquidity_transformer.package_hash()),
            "amount" => U256::from(AMOUNT)
        },
        now(),
    );
    let investor_balance: U256 = liquidity_transformer
        .query_dictionary("investor_balance", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(
        investor_balance,
        0.into(),
        "Investor has already some wise balance"
    );
    const DAYS: u64 = 12;
    const TIME: u64 = DAYS * 86400 * 1000;
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "reserve_wise_with_token",
            "token_address" => Key::Hash(erc20.package_hash()),
            "token_amount" => U256::from(AMOUNT),
            "investment_mode" => investment_mode,
        },
        now() + TIME,
    );
    let investor_balance: U256 = liquidity_transformer
        .query_dictionary("investor_balance", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(
        investor_balance,
        99690060.into(), // Not exactly equal to AMOUNT due to fee cutting during 'swap_exact_tokens_for_cspr'
        "Investor wise balance not increased"
    );
}

#[test]
fn test_forward_liquidity() {
    let (
        env,
        liquidity_transformer,
        owner,
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
    initialize_flow(
        &env,
        &liquidity_transformer,
        owner,
        uniswap_router,
        uniswap_pair,
        &wise,
        &scspr,
        uniswap_factory,
        helper,
    );
    // Using session code as caller of purse is required for reserving wise
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
    let uniswap_swaped: bool = liquidity_transformer
        .query_dictionary("globals", "uniswap_swaped".into())
        .unwrap_or_default();
    assert!(
        !uniswap_swaped,
        "Reserved tokens equivalent to CSPR contributed already forwarded"
    );
    // Forward liquidity to be done after investment days
    const INVESTMENT_DAY: u64 = 20; // Some random day after investment days passed
    const INVESTMENT_DAY_TIME: u64 = INVESTMENT_DAY * 86400 * 1000;
    liquidity_transformer.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair_wise.package_hash())
        },
        now() + INVESTMENT_DAY_TIME,
    );
    let uniswap_swaped: bool = liquidity_transformer
        .query_dictionary("globals", "uniswap_swaped".into())
        .unwrap_or_default();
    assert!(
        uniswap_swaped,
        "Reserved tokens equivalent to CSPR contributed not forwarded"
    );
}

#[test]
fn test_payout_investor_address() {
    let (
        env,
        liquidity_transformer,
        owner,
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
    initialize_flow(
        &env,
        &liquidity_transformer,
        owner,
        uniswap_router,
        uniswap_pair,
        &wise,
        &scspr,
        uniswap_factory,
        helper,
    );
    // Using session code as caller of purse is required for reserving wise
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
    // Forward liquidity to be done after investment days
    const INVESTMENT_DAY: u64 = 20; // Some random day after investment days passed
    const INVESTMENT_DAY_TIME: u64 = INVESTMENT_DAY * 86400 * 1000;
    liquidity_transformer.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair_wise.package_hash())
        },
        now() + INVESTMENT_DAY_TIME,
    );
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "payout_investor_address",
            "investor_address" => Key::Account(owner)
        },
        now() + INVESTMENT_DAY_TIME,
    );
    let ret: U256 = session_code_result(&env, owner);
    assert_eq!(ret, 2640002000000000u64.into()); // calculated amount in contract
}

#[test]
fn test_get_my_tokens() {
    let (
        env,
        liquidity_transformer,
        owner,
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
    initialize_flow(
        &env,
        &liquidity_transformer,
        owner,
        uniswap_router,
        uniswap_pair,
        &wise,
        &scspr,
        uniswap_factory,
        helper,
    );
    // Using session code as caller of purse is required for reserving wise
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
    // Forward liquidity to be done after investment days
    const INVESTMENT_DAY: u64 = 20; // Some random day after investment days passed
    const INVESTMENT_DAY_TIME: u64 = INVESTMENT_DAY * 86400 * 1000;
    liquidity_transformer.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair_wise.package_hash())
        },
        now() + INVESTMENT_DAY_TIME,
    );
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(balance, 0.into(), "Already have some wise tokens");
    liquidity_transformer.call_contract(
        owner,
        "get_my_tokens",
        runtime_args! {},
        now() + INVESTMENT_DAY_TIME,
    );
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(
        balance,
        2640002000000000u64.into(), // calculated amount in contract
        "Tokens not transfered to owner"
    );
}

#[test]
fn test_prepare_path() {
    let (env, liquidity_transformer, owner, erc20, wcspr, _, _, _, _, _, _, _) = deploy();
    let token_address: Key = Key::Hash(erc20.package_hash());
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "prepare_path",
            "token_address" => token_address,
        },
        now(),
    );
    let ret: Vec<Key> = session_code_result(&env, owner);
    assert_eq!(ret[0], Key::Hash(erc20.package_hash()));
    assert_eq!(ret[1], Key::Hash(wcspr.package_hash()));
}

#[test]
fn test_request_refund() {
    let (env, liquidity_transformer, owner, _, _, _, _, _, _, _, _, _) = deploy();
    // Using session code as caller of purse is required for reserving wise
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
    // TIME PASSED, NOW CAN REFUND
    const DAYS: u64 = 30;
    const TIME: u64 = DAYS * 86400 * 1000;
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "request_refund"
        },
        now() + TIME,
    );
    let ret: (U256, U256) = session_code_result(&env, owner);
    assert_eq!(
        ret,
        (
            <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(TWOTHOUSEND_CSPR),
            2640002000000000u64.into() // calculated amount in contract
        ),
        "Invalid refund"
    );
}
