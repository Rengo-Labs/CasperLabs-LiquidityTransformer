use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256, U512};
use test_env::{TestContract, TestEnv};

use crate::scspr_instance::SCSPRInstance;

const TIME: u64 = 300_000_000;
const PURSE_AMOUNT: U512 = U512([10_000_000_000_000,0,0,0,0,0,0,0]);
const INVESTMENT_AMOUNT: U256 = U256([2_000_000_000_000, 0, 0, 0]);
const INVESTMENT_AMOUNT_U512: U512 = U512([2_000_000_000_000, 0, 0, 0, 0, 0, 0, 0]);

pub fn deploy_liquidity_transformer(
    env: &TestEnv,
    owner: AccountHash,
    wise_token: Key,
    scspr: Key,
    uniswap_pair: Key,
    uniswap_router: Key,
    wcspr: Key,
    amount: U512,
) -> TestContract {
    TestContract::new(
        env,
        "liquidity_transformer.wasm",
        "liquidity_transformer",
        owner,
        runtime_args! {
            "wise_token" => wise_token,
            "scspr" => scspr,
            "uniswap_pair" => uniswap_pair,
            "uniswap_router" => uniswap_router,
            "wcspr" => wcspr,
            "amount" => amount
        },
        0,
    )
}

pub fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    let decimals: u8 = 18;
    let initial_supply: U256 = 0.into();
    TestContract::new(
        env,
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

pub fn deploy_uniswap_factory(
    env: &TestEnv,
    owner: AccountHash,
    fee_to_setter: Key,
) -> TestContract {
    TestContract::new(
        env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => fee_to_setter
        },
        0,
    )
}

pub fn deploy_wcspr(env: &TestEnv, owner: AccountHash) -> TestContract {
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

pub fn deploy_flash_swapper(
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

pub fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
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
        0,
    )
}

pub fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        0,
    )
}

pub fn deploy_uniswap_router(
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

pub fn deploy_transfer_helper(
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
            "transfer_invoker" => transfer_invoker,
        },
        0,
    )
}

pub fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash) -> TestContract {
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
pub fn deploy_wise_token(
    env: &TestEnv,
    owner: AccountHash,
    scspr: &TestContract,
    router: &TestContract,
    factory: &TestContract,
    pair: &TestContract,
    liquidity_guard: &TestContract,
    wcspr: &TestContract,
) -> TestContract {
    TestContract::new(
        env,
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
            "launch_time" => U256::from(0),
        },
        0,
    )
}

fn deploy_scspr(
    env: &TestEnv,
    owner: AccountHash,
) -> (
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) {
    let wcspr = deploy_wcspr(&env, owner);
    let uniswap_library = deploy_uniswap_library(&env, owner);
    let uniswap_factory = deploy_uniswap_factory(&env, owner, Key::Account(owner));
    let uniswap_router =
        deploy_uniswap_router(&env, owner, &uniswap_factory, &wcspr, &uniswap_library);
    let erc20 = deploy_erc20(&env, owner);
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &uniswap_factory);
    let uniswap_pair: TestContract =
        deploy_uniswap_pair(&env, owner, &flash_swapper, &uniswap_factory);
    let scspr = SCSPRInstance::new(
        &env,
        "scspr",
        owner,
        Key::Hash(wcspr.package_hash()),
        Key::Hash(uniswap_pair.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(uniswap_factory.package_hash()),
        PURSE_AMOUNT
    );
    let helper = deploy_transfer_helper(&env, owner, Key::Hash(scspr.package_hash()));
    (
        scspr,
        uniswap_router,
        uniswap_factory,
        uniswap_pair,
        wcspr,
        erc20,
        helper,
    )
}

#[allow(clippy::type_complexity)]
pub fn initialize_system()
// -> (
//     TestEnv,
//     TestContract,
//     TestContract,
//     AccountHash,
//     TestContract,
//     TestContract,
//     TestContract,
//     TestContract,
//     TestContract,
//     TestContract,
//     TestContract,
//     TestContract,
//     TestContract,
// )
{
    let env = TestEnv::new();
    let owner = env.next_user();
    let (scspr, uniswap_router, uniswap_factory, uniswap_pair, wcspr, erc20, helper) =
        deploy_scspr(&env, owner);
    let liquidity_guard = deploy_liquidity_guard(&env, owner);
    let token = deploy_wise_token(
        &env,
        owner,
        &scspr,
        &uniswap_router,
        &uniswap_factory,
        &uniswap_pair,
        &liquidity_guard,
        &wcspr,
    );
    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args!{
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
    scspr.call_contract(
        owner,
        "create_pair",
        runtime_args! {
            "pair" => Key::Hash(uniswap_pair.package_hash()),
        },
        0,
    );
    token.call_contract(owner, "create_pair", runtime_args! {}, 0);
    let pair: Key = token.query_named_key("uniswap_pair_contract_hash".into());
    scspr.call_contract(
        owner,
        "get_synthetic_balance_js_client",
        runtime_args! {},
        0,
    );
    let synthetic_balance_before: U256 = scspr.query_named_key("result".into());
    let lt = deploy_liquidity_transformer(
        &env,
        owner,
        Key::Hash(token.package_hash()),
        Key::Hash(scspr.package_hash()),
        Key::Hash(uniswap_pair.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(wcspr.package_hash()),
        PURSE_AMOUNT
    );
    
    // Using session code as creation of purse is required for transformer
    TestContract::new(
        &env,
        "session-code-scspr.wasm",
        "liquidity_transfomer_setup",
        owner,
        runtime_args! {
            "entrypoint" => "set_liquidity_transfomer",
            "package_hash" => Key::Hash(token.package_hash()),
            "immutable_transformer" => Key::Hash(lt.package_hash())
        },
        0,
    );

    // NOW CALLS TIME SHOULD BE IN ADVANCED TIME 'SECONDS_IN_DAY'

    // Using session code as caller of purse is required for reserving wise
    TestContract::new(
        &env,
        "session-code-scspr.wasm",
        "reserve_wise_call",
        owner,
        runtime_args! {
            "entrypoint" => "reserve_wise",
            "package_hash" => Key::Hash(lt.package_hash()),
            "investment_mode" => 1_u8,
            "msg_value" => INVESTMENT_AMOUNT,
            "amount" => INVESTMENT_AMOUNT_U512,
        },
        TIME,
    );

    lt.call_contract(owner, "forward_liquidity", runtime_args! {}, TIME * 150_000);

    //   await advanceTimeAndBlock(15 * SECONDS_IN_DAY);
    //   await lt.forwardLiquidity({ from: person });
    //   wrappedBalanceAfter = await sbnb.getWrappedBalance();
    //   syntheticBalanceAfter = await sbnb.getSyntheticBalance();
    //   await lt.$getMyTokens({ from: person });
    //   wbnbAddress = await sbnb.WBNB();
    //   pairAddress = await sbnb.PANCAKE_PAIR();
    //   routerAddress = await sbnb.PANCAKE_ROUTER();
    //   wrapped = await WETHInterface.at(wbnbAddress);
    //   router = await RouterInterface.at(routerAddress);
    //   bep20 = await BEP20Interface.at(pairAddress);
    //   const balanceOfWBNB = await wrapped.balanceOf(pairAddress);
    //   assert.equal(syntheticBalanceAfter.toString(), balanceOfWBNB.toString());
    //   assert.equal(wrappedBalanceAfter.toString(), balanceOfWBNB.toString());

    //
    //
    //

    // let proxy = SCSPRInstance::proxy(&env, "proxy", owner, Key::Hash(scspr.package_hash()));

    // let liquidity_transformer = deploy_liquidity_transformer(
    //     &env,
    //     owner,
    //     &wise,
    //     &scspr,
    //     &uniswap_pair,
    //     &uniswap_router,
    //     &wcspr,
    // );

    // (
    //     env,
    //     proxy,
    //     scspr,
    //     owner,
    //     erc20,
    //     uniswap_pair,
    //     transfer_helper,
    //     wise,
    //     synthetic_token,
    //     uniswap_router,
    //     uniswap_factory,
    //     wcspr,
    //     liquidity_transformer,
    // )
}

#[test]
fn make_test() {
    initialize_system();
}
