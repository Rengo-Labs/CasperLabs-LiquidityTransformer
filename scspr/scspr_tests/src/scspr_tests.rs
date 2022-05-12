use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::scspr_instance::SCSPRInstance;

pub fn deploy_forward_liquidity_purse_proxy(
    env: &TestEnv,
    sender: AccountHash,
    destination_package_hash: Key,
    destination_entrypoint: &str,
    amount: U512,
    block_time: u64,
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
        block_time,
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

pub fn deploy_form_liquidity_purse_proxy(
    env: &TestEnv,
    sender: AccountHash,
    destination_package_hash: Key,
    destination_entrypoint: &str,
    amount: U512,
    pair: Key,
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
            "pair" => pair
        },
        0,
    )
}

fn deploy_liquidity_transformer(
    env: &TestEnv,
    owner: AccountHash,
    wise: &TestContract,
    scspr: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    wcspr: &TestContract,
) -> TestContract {
    TestContract::new(
        env,
        "liquidity_transformer.wasm",
        "liquidity_transformer",
        owner,
        runtime_args! {
            "wise_token" => Key::Hash(wise.package_hash()),
            "scspr" => Key::Hash(scspr.package_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.package_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
        },
        0,
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

fn deploy_synthetic_token(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    erc20: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "synthetic_token.wasm",
        "synthetic_token",
        owner,
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.package_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.package_hash()),
            "erc20" => erc20,
        },
        0,
    )
}

fn deploy_transfer_helper(env: &TestEnv, owner: AccountHash, scspr: &TestContract) -> TestContract {
    TestContract::new(
        &env,
        "transfer_helper.wasm",
        "transfer_helper",
        owner,
        runtime_args! {
            "transfer_invoker" => Key::Hash(scspr.package_hash()),
        },
        0,
    )
}

fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "liquidity_guard.wasm",
        "liquidity_guard",
        owner,
        runtime_args! {},
        0,
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
            "scspr" => Key::Hash(scspr.package_hash()),
            "router" => Key::Hash(router.package_hash()),
            "factory" => Key::Hash(factory.package_hash()),
            "pair" => Key::Hash(pair.package_hash()),
            "liquidity_guard" => Key::Hash(liquidity_guard.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "erc20" => Key::Hash(erc20.package_hash()),
            "launch_time" => U256::from(0),
        },
        0,
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
        Key::Hash(erc20.package_hash()),
    );

    let scspr = SCSPRInstance::new(
        &env,
        "scspr",
        owner,
        Key::Hash(wcspr.package_hash()),
        Key::Hash(uniswap_pair.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(uniswap_factory.package_hash()),
        Key::Hash(synthetic_token.package_hash()),
    );
    let transfer_helper = deploy_transfer_helper(&env, owner, &scspr);

    let proxy = SCSPRInstance::proxy(&env, "proxy", owner, Key::Hash(scspr.package_hash()));

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

// #[test]
// fn test_scspr_deploy() {
//     let (_, _, _, _, _, _, _, _, _, _, _, _, _) = deploy();
// }

// #[test]
// fn test_scspr_deposit() {
//     let (
//         env,
//         proxy,
//         scspr,
//         owner,
//         erc20,
//         uniswap_pair,
//         _,
//         wise,
//         _,
//         uniswap_router,
//         uniswap_factory,
//         wcspr,
//         liquidity_transformer,
//     ) = deploy();

//     // let proxy_instance = SCSPRInstance::instance(proxy.clone());
//     // proxy_instance.temp_purse(owner, Key::Hash(proxy.package_hash()));
//     // let purse: URef = proxy.query_named_key("result".to_string());

//     // forward_liquidity(
//     //     &env,
//     //     liquidity_transformer,
//     //     owner,
//     //     proxy.clone(),
//     //     erc20.clone(),
//     //     uniswap_router,
//     //     uniswap_pair.clone(),
//     //     wise,
//     //     scspr.clone(),
//     //     uniswap_factory,
//     //     wcspr.clone(),
//     // );

//     // erc20.call_contract(
//     //     owner,
//     //     "mint",
//     //     runtime_args! {
//     //         "to" => Key::Hash(uniswap_pair.package_hash()),
//     //         "amount" => U256::from(10000000000000000000 as u128)
//     //     },
//     //     0,
//     // );

//     // // MINTING TO A CONTRACT WITH WCSPR
//     // wcspr.call_contract(
//     //     owner,
//     //     "deposit_no_return",
//     //     runtime_args! {
//     //         "purse" => purse,
//     //         "amount" => U512::from(1000000000)
//     //     },
//     //     0,
//     // );
//     // wcspr.call_contract(
//     //     owner,
//     //     "approve",
//     //     runtime_args! {
//     //         "spender" => Key::Hash(proxy.query_named_key("package_hash".to_string())),
//     //         "amount" => U256::from(1000000000)
//     //     },
//     //     0,
//     // );
//     // proxy.call_contract(
//     //     owner,
//     //     "transfer_from",
//     //     runtime_args! {
//     //         "wcspr" => Key::Hash(wcspr.package_hash()),
//     //         "owner" => Key::Account(owner),
//     //         "recipient" => Key::Hash(uniswap_pair.query_named_key("self_package_hash".to_string())),
//     //         "amount" => U256::from(1000000000),
//     //     },
//     //     0,
//     // );
//     // // MINTING TO A CONTRACT WITH WCSPR

//     forward_liquidity(
//         &env,
//         liquidity_transformer,
//         owner,
//         proxy,
//         erc20,
//         uniswap_router,
//         uniswap_pair,
//         wise,
//         scspr,
//         uniswap_factory,
//         wcspr,
//     );

//     // let _: TestContract = deploy_deposit_purse_proxy(
//     //     &env,
//     //     owner,
//     //     Key::Hash(scspr.package_hash()),
//     //     "deposit",
//     //     U512::from(10),
//     // );
// }

// // #[test]
// fn test_scspr_withdraw() {
//     let (_, proxy, scspr, owner, erc20, uniswap_pair, _, _, _, _, _, _, _) = deploy();

//     let msg_value: U256 = 100.into();
//     let succesor_purse: URef = URef::new(erc20.package_hash(), None.unwrap_or_default());

//     let scspr = SCSPRInstance::instance(scspr);
//     let proxy = SCSPRInstance::instance(proxy);

//     scspr.forward_ownership(owner, Key::Hash(erc20.package_hash()));
//     proxy.form_liquidity(owner, Key::Hash(uniswap_pair.package_hash()));
//     scspr.withdraw(owner, msg_value, succesor_purse);
// }

// #[test]
// fn test_scspr_create_pair() {
//     let (_, _, scspr, owner, _, uniswap_pair, _, _, _, _, _, _, _) = deploy();

//     let scspr = SCSPRInstance::instance(scspr);

//     scspr.forward_ownership(owner, Key::Account(owner));
//     scspr.create_pair(owner, Key::Hash(uniswap_pair.package_hash()));
// }

// #[test]
// fn test_scspr_define_helper() {
//     let (_, proxy, _, owner, _, _, transfer_helper, _, _, _, _, _, _) = deploy();

//     let proxy = SCSPRInstance::instance(proxy);

//     proxy.define_helper(owner, Key::Hash(transfer_helper.package_hash()));
// }

// #[test]
// fn test_scspr_define_token() {
//     let (_, proxy, _, owner, _, _, _, wise, _, _, _, _, _) = deploy();

//     let proxy = SCSPRInstance::instance(proxy);

//     proxy.define_token(owner, Key::Hash(wise.package_hash()));
// }
