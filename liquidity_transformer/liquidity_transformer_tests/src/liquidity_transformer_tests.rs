use crate::liquidity_transformer_instance::*;
use casper_types::{runtime_args, Key, RuntimeArgs, U256, U512};
use num_traits::cast::AsPrimitive;

#[test]
fn test_deploy() {
    let (_, _, _, _, _, _, _, _, _, _, _, _, _, _) = deploy();
}

#[test]
fn test_current_stakeable_day() {
    let (env, lt, owner, _, _, _, _, _, _, _, _, _, _, _) = deploy();
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
    let ret: u64 = session_code_result(&env, owner, "current_stakeable_day");
    assert_eq!(ret - 2, DAYS, "Invalid stakeable day"); // - 2 for past launch time balance
}

#[test]
fn test_set_settings() {
    let (
        _,
        liquidity_transformer,
        owner,
        _,
        _,
        _,
        pair_scspr,
        wise,
        scspr,
        _,
        pair_stakeable,
        _,
        _,
        _,
    ) = deploy();
    liquidity_transformer.call_contract(
        owner,
        "set_settings",
        runtime_args! {
            "wise_token" =>  Key::Hash(wise.package_hash()),
            "pair_wise" => Key::Hash(pair_stakeable.package_hash()),
            "pair_scspr" => Key::Hash(pair_scspr.package_hash()),
            "uniswap_pair" => Key::Hash(pair_stakeable.package_hash()),
            "synthetic_cspr" => Key::Hash(scspr.package_hash())
        },
        now(),
    );
    let setted_wise_contract: Key =
        liquidity_transformer.query_named_key("wise_contract".to_string());
    let setted_pair_wise: Key = liquidity_transformer.query_named_key("pair_wise".to_string());
    let setted_pair_scspr: Key = liquidity_transformer.query_named_key("pair_scspr".to_string());
    let setted_scspr: Key = liquidity_transformer.query_named_key("scspr".to_string());
    assert_eq!(
        setted_wise_contract,
        Key::Hash(wise.package_hash()),
        "wise address not set"
    );
    assert_eq!(
        setted_pair_wise,
        Key::Hash(pair_stakeable.package_hash()),
        "pair wise address not set"
    );
    assert_eq!(
        setted_pair_scspr,
        Key::Hash(pair_scspr.package_hash()),
        "pair scspr address not set"
    );
    assert_eq!(
        setted_scspr,
        Key::Hash(scspr.package_hash()),
        "scspr address not set"
    );
}

#[test]
fn test_renounce_keeper() {
    let (_, liquidity_transformer, owner, _, _, _, _, _, _, _, _, _, _, _) = deploy();
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
    let (env, liquidity_transformer, owner, _, _, _, _, _, _, _, _, _, _, _) = deploy();
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
        _,
        _,
        _,
        uniswap_factory,
        _,
        flash_swapper,
        _,
        time,
    ) = deploy();
    let stable_usd_wcspr_pair = deploy_uniswap_pair(
        &env,
        owner,
        "pair-3",
        "stable_usd_wcspr_pair".into(),
        "SUWP".into(),
        9,
        0.into(),
        &flash_swapper,
        &uniswap_factory,
        time,
    );
    uniswap_factory.call_contract(
        owner,
        "create_pair",
        runtime_args! {
            "token_a" => Key::Hash(erc20.package_hash()),
            "token_b" => Key::Hash(wcspr.package_hash()),
            "pair_hash" => Key::Hash(stable_usd_wcspr_pair.package_hash()),
        },
        0,
    );
    add_liquidity(
        &env,
        owner,
        &erc20,
        &uniswap_router,
        &stable_usd_wcspr_pair,
        &wcspr,
        time,
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
        98715803.into(), // Not exactly equal to AMOUNT due to fee cutting during 'swap_exact_tokens_for_cspr'
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
        _,
        wise,
        scspr,
        uniswap_factory,
        _,
        _,
        _,
        time,
    ) = deploy();
    let uniswap_swaped: bool = liquidity_transformer
        .query_dictionary("globals", "uniswap_swaped".into())
        .unwrap_or_default();
    assert!(
        !uniswap_swaped,
        "Reserved tokens equivalent to CSPR contributed already forwarded"
    );
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
    forward_liquidity(&env, &liquidity_transformer, owner, &wise, &scspr, time);
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
        _,
        wise,
        scspr,
        uniswap_factory,
        _,
        _,
        _,
        time,
    ) = deploy();
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
    let time = forward_liquidity(&env, &liquidity_transformer, owner, &wise, &scspr, time);
    session_code_call(
        &env,
        owner,
        runtime_args! {
            "package_hash" => Key::Hash(liquidity_transformer.package_hash()),
            "entrypoint" => "payout_investor_address",
            "investor_address" => Key::Account(owner)
        },
        time,
    );
    let ret: U256 = session_code_result(&env, owner, "payout_investor_address");
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
        _,
        wise,
        scspr,
        uniswap_factory,
        _,
        _,
        _,
        time,
    ) = deploy();
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
    let time = forward_liquidity(&env, &liquidity_transformer, owner, &wise, &scspr, time);
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(balance, 0.into(), "Already have some wise tokens");
    liquidity_transformer.call_contract(owner, "get_my_tokens", runtime_args! {}, time);
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
    let (env, liquidity_transformer, owner, erc20, wcspr, _, _, _, _, _, _, _, _, _) = deploy();
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
    let ret: Vec<Key> = session_code_result(&env, owner, "prepare_path");
    assert_eq!(ret[0], Key::Hash(erc20.package_hash()));
    assert_eq!(ret[1], Key::Hash(wcspr.package_hash()));
}

#[test]
fn test_request_refund() {
    let (env, liquidity_transformer, owner, _, _, _, _, _, _, _, _, _, _, _) = deploy();
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
    let ret: (U256, U256) = session_code_result(&env, owner, "request_refund");
    assert_eq!(
        ret,
        (
            <casper_types::U512 as AsPrimitive<casper_types::U256>>::as_(TWOTHOUSEND_CSPR),
            2640002000000000u64.into() // calculated amount in contract
        ),
        "Invalid refund"
    );
}
