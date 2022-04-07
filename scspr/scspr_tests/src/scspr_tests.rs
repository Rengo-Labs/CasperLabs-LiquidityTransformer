use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::scspr_instance::SCSPRInstance;

fn deploy_liquidity_transformer(
    env: &TestEnv,
    owner: AccountHash,
    wise: &TestContract,
    scspr: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    wcspr: &TestContract,
    uniswap_router_package: Key,
) -> TestContract {
    TestContract::new(
        env,
        "liquidity_transformer.wasm",
        "liquidity_transformer",
        owner,
        runtime_args! {
            "wise_token" => Key::Hash(wise.contract_hash()),
            "scspr" => Key::Hash(scspr.contract_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.contract_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "uniswap_router_package" => uniswap_router_package
        },
    )
}

fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    let decimals: u8 = 18;
    let initial_supply: U256 = 0.into();
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "initial_supply" => initial_supply,
            "name" => "ERC-20",
            "symbol" => "ERC",
            "decimals" => decimals
        },
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
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "dai" => Key::Hash(wcspr.contract_hash()),
            "uniswap_v2_factory" => Key::Hash(uniswap_factory.contract_hash())
        },
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
            "factory_hash" => Key::Hash(uniswap_factory.contract_hash()),
        },
    )
}

fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
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
            "factory" => Key::Hash(uniswap_factory.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "library" => Key::Hash(uniswap_library.contract_hash())
        },
    )
}

fn deploy_synthetic_token(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    erc20: Key,
    uniswap_router_package: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "synthetic_token.wasm",
        "synthetic_token",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.contract_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.contract_hash()),
            "erc20" => erc20,
            "uniswap_router_package" => uniswap_router_package
        },
    )
}

fn deploy_transfer_helper(env: &TestEnv, owner: AccountHash, scspr: &TestContract) -> TestContract {
    TestContract::new(
        &env,
        "transfer_helper.wasm",
        "transfer_helper",
        owner,
        runtime_args! {
            "transfer_invoker" => Key::Hash(scspr.contract_hash()),
        },
    )
}

fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "liquidity_guard.wasm",
        "liquidity_guard",
        owner,
        runtime_args! {},
    )
}

fn deploy_wise_token(
    env: &TestEnv,
    owner: AccountHash,
    scspr: &TestContract,
    router: &TestContract,
    factory: &TestContract,
    pair: &TestContract,
    liquidity_guard: &TestContract,
    wcspr: &TestContract,
    erc20: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "stakeabletoken.wasm",
        "wisetoken",
        owner,
        runtime_args! {
            "scspr" => Key::Hash(scspr.contract_hash()),
            "router" => Key::Hash(router.contract_hash()),
            "factory" => Key::Hash(factory.contract_hash()),
            "pair" => Key::Hash(pair.contract_hash()),
            "liquidity_guard" => Key::Hash(liquidity_guard.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "erc20" => Key::Hash(erc20.contract_hash()),
            "launch_time" => U256::from(0),
        },
    )
}

fn deploy() -> (
    TestEnv,
    TestContract,
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

    let _wcspr = deploy_wcspr(&env, owner);
    let wcspr = deploy_wcspr(&env, owner);
    let uniswap_library = deploy_uniswap_library(&env, owner);
    let uniswap_factory = deploy_uniswap_factory(&env, owner);
    let uniswap_router =
        deploy_uniswap_router(&env, owner, &uniswap_factory, &wcspr, &uniswap_library);
    let uniswap_router_package: ContractPackageHash =
        uniswap_router.query_named_key("package_hash".to_string());
    let erc20 = deploy_erc20(&env, owner);
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &uniswap_factory);
    let uniswap_pair: TestContract =
        deploy_uniswap_pair(&env, owner, &flash_swapper, &uniswap_factory);

    let synthetic_token = deploy_synthetic_token(
        &env,
        owner,
        &_wcspr,
        &uniswap_pair,
        &uniswap_router,
        Key::Hash(erc20.contract_hash()),
        Key::from(uniswap_router_package),
    );

    let scspr = SCSPRInstance::new(
        &env,
        "scspr",
        owner,
        Key::Hash(uniswap_factory.contract_hash()),
        Key::Hash(synthetic_token.contract_hash()),
    );
    let transfer_helper = deploy_transfer_helper(&env, owner, &scspr);

    let proxy = SCSPRInstance::proxy(&env, "proxy", owner, Key::Hash(scspr.contract_hash()));

    // Extra cotnracts needed for functionalities
    let liquidity_guard = deploy_liquidity_guard(&env, owner);
    let wise = deploy_wise_token(
        &env,
        owner,
        &scspr,
        &uniswap_router,
        &uniswap_factory,
        &uniswap_pair,
        &liquidity_guard,
        &wcspr,
        &erc20,
    );

    let liquidity_transformer = deploy_liquidity_transformer(
        &env,
        owner,
        &wise,
        &scspr,
        &uniswap_pair,
        &uniswap_router,
        &wcspr,
        Key::from(uniswap_router_package),
    );

    (
        env,
        proxy,
        scspr,
        owner,
        erc20,
        uniswap_pair,
        transfer_helper,
        wise,
        synthetic_token,
        uniswap_router,
        uniswap_factory,
        wcspr,
        liquidity_transformer,
    )
}

fn forward_liquidity(
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
    // EXT
    uniswap_pair.call_contract(
        owner,
        "initialize",
        runtime_args! {
            "token0" => Key::Hash(erc20.contract_hash()),
            "token1" => Key::Hash(erc20.contract_hash()),
            "factory_hash" => Key::Hash(uniswap_factory.contract_hash()),
        },
    );
    // EXT

    let uniswap_pair_package: ContractPackageHash =
        uniswap_pair.query_named_key("self_package_hash".to_string());

    const DAYS: u64 = 16;
    const TIME: u64 = DAYS * 86400 * 1000;

    const MINTED: u128 = 45;

    let proxy_key: Key = Key::Hash(proxy.contract_hash());

    let proxy_instance = SCSPRInstance::instance(proxy.clone());

    proxy_instance.temp_purse(owner, proxy_key);
    let purse: URef = proxy_instance.result();

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::from(uniswap_pair_package),
            "amount" => U256::from(MINTED)
        },
    );

    let uniswap_router_package: ContractPackageHash =
        uniswap_router.query_named_key("package_hash".to_string());
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::from(uniswap_router_package)
        },
    );

    // synthetic_helper.call_contract(
    //     owner,
    //     "fund_contract",
    //     runtime_args! {
    //         "caller_purse" => purse,
    //         "amount" => U512::from(10000000000 as u128)
    //     },
    // );

    let liquidity_package: Key =
        liquidity_contract.query_named_key("self_package_hash".to_string());
    wise.call_contract(
        owner,
        "set_liquidity_transfomer",
        runtime_args! {
            "immutable_transformer" => liquidity_package,
            "transformer_purse" => purse
        },
    );

    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args! {
            "wise" => Key::Hash(wise.contract_hash())
        },
    );

    let scspr_package: ContractPackageHash = scspr.query_named_key("self_package_hash".to_string());
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "amount" => U256::from(MINTED),
            "to" => Key::from(scspr_package)
        },
    );

    let uniswap_router_package: ContractPackageHash =
        uniswap_router.query_named_key("package_hash".to_string());
    scspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "amount" => U256::from(MINTED),
            "spender" => Key::from(uniswap_router_package)
        },
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
    );

    scspr.call_contract(
        owner,
        "mint",
        runtime_args! {
            "recipient" => Key::Hash(scspr.contract_hash()),
            "amount" => U256::from(AMOUNT)
        },
    );

    wcspr.call_contract(
        owner,
        "deposit_no_return",
        runtime_args! {
            "purse" => purse,
            "amount" => U512::from(MINTED)
        },
    );

    wise.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => liquidity_package,
            "amount" => U256::from(AMOUNT)
        },
    );

    let liquidity: Key = Key::Hash(liquidity_contract.contract_hash());

    let investment_mode: u8 = 1;
    let msg_value: U256 = (75757576000000000 as u128).into();

    const _DAYS: u64 = 15;
    const _TIME: u64 = _DAYS * 86400 * 1000;

    proxy_instance.reserve_wise(owner, liquidity, investment_mode, msg_value);

    liquidity_contract.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {
            "purse" => purse
        },
    );
}

#[test]
fn test_scspr_deploy() {
    let (_, _, _, _, _, _, _, _, _, _, _, _, _) = deploy();
}

// #[test]
fn test_form_liquidity() {
    let (_, proxy, _, owner, _, uniswap_pair, _, wise, _, _, _, _, _) = deploy();

    let proxy = SCSPRInstance::instance(proxy);

    proxy.forward_ownership(owner, Key::from(proxy.proxy_package_hash()));
    proxy.set_wise(owner, Key::Hash(wise.contract_hash()));
    proxy.form_liquidity(owner, Key::Hash(uniswap_pair.contract_hash()));
}

// #[test]
fn test_scspr_deposit() {
    let (
        _,
        proxy,
        scspr,
        owner,
        erc20,
        uniswap_pair,
        _,
        wise,
        _,
        uniswap_router,
        uniswap_factory,
        wcspr,
        liquidity_transformer,
    ) = deploy();

    let proxy_instance = SCSPRInstance::instance(proxy.clone());
    proxy_instance.temp_purse(owner, Key::Hash(proxy.contract_hash()));
    let purse: URef = proxy.query_named_key("result".to_string());
    let msg_value: U256 = 10.into();

    forward_liquidity(
        liquidity_transformer,
        owner,
        proxy.clone(),
        erc20.clone(),
        uniswap_router,
        uniswap_pair.clone(),
        wise,
        scspr.clone(),
        uniswap_factory,
        wcspr.clone(),
    );

    let uniswap_pair_package: ContractPackageHash =
        uniswap_pair.query_named_key("self_package_hash".to_string());
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::from(uniswap_pair_package),
            "amount" => U256::from(10000000000000000000 as u128)
        },
    );

    // MINTING TO A CONTRACT WITH WCSPR
    wcspr.call_contract(
        owner,
        "deposit_no_return",
        runtime_args! {
            "purse" => purse,
            "amount" => U512::from(1000000000)
        },
    );
    wcspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(proxy.query_named_key("package_hash".to_string())),
            "amount" => U256::from(1000000000)
        },
    );
    proxy.call_contract(
        owner,
        "transfer_from",
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "owner" => Key::Account(owner),
            "recipient" => Key::Hash(uniswap_pair.query_named_key("self_package_hash".to_string())),
            "amount" => U256::from(1000000000),
        },
    );
    // MINTING TO A CONTRACT WITH WCSPR

    let scspr = SCSPRInstance::instance(scspr);
    scspr.deposit(owner, msg_value, purse);
}

// #[test]
fn test_scspr_withdraw() {
    let (_, proxy, scspr, owner, erc20, uniswap_pair, _, _, _, _, _, _, _) = deploy();

    let msg_value: U256 = 100.into();
    let succesor_purse: URef = URef::new(erc20.contract_hash(), None.unwrap_or_default());

    let scspr = SCSPRInstance::instance(scspr);
    let proxy = SCSPRInstance::instance(proxy);

    scspr.forward_ownership(owner, Key::Hash(erc20.contract_hash()));
    proxy.form_liquidity(owner, Key::Hash(uniswap_pair.contract_hash()));
    scspr.withdraw(owner, msg_value, succesor_purse);
}

// #[test]
fn test_scspr_create_pair() {
    let (_, _, scspr, owner, _, uniswap_pair, _, _, _, _, _, _, _) = deploy();

    let scspr = SCSPRInstance::instance(scspr);

    scspr.forward_ownership(owner, Key::Account(owner));
    scspr.create_pair(owner, Key::Hash(uniswap_pair.contract_hash()));
}

// #[test]
fn test_scspr_define_helper() {
    let (_, proxy, _, owner, _, _, transfer_helper, _, _, _, _, _, _) = deploy();

    let proxy = SCSPRInstance::instance(proxy);

    proxy.define_helper(owner, Key::Hash(transfer_helper.contract_hash()));
}

// #[test]
fn test_scspr_define_token() {
    let (_, proxy, _, owner, _, _, _, wise, _, _, _, _, _) = deploy();

    let proxy = SCSPRInstance::instance(proxy);

    proxy.define_token(owner, Key::Hash(wise.contract_hash()));
}
