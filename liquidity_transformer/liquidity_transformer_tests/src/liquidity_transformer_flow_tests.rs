use crate::liquidity_transformer_instance::*;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};

#[test]
fn test_reserve_claim_flow() {
    let (
        env,
        liquidity_transformer,
        owner,
        _,
        _,
        uniswap_router,
        _,
        wise,
        scspr,
        uniswap_factory,
        _,
        _,
        _,
        time,
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

    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(uniswap_router.package_hash())
        },
        time,
    );

    let time = forward_liquidity(&env, &liquidity_transformer, owner, &wise, &scspr, time);

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
