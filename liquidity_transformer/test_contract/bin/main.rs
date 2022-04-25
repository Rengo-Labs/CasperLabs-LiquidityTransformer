#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec};

use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256, U512,
};

pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let liquidity_transformer: Key = runtime::get_named_arg("liquidity_transformer");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::liquidity_transformer_key(),
        ContractPackageHash::from(liquidity_transformer.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn set_settings() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());

    let wise_token: Key = runtime::get_named_arg("wise_token");
    let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
    let synthetic_cspr: Key = runtime::get_named_arg("synthetic_cspr");

    let () = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "set_settings",
        runtime_args! {
            "wise_token" => wise_token,
            "uniswap_pair" => uniswap_pair,
            "synthetic_cspr" => synthetic_cspr
        },
    );
}

#[no_mangle]
fn renounce_keeper() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());

    let () = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "renounce_keeper",
        runtime_args! {},
    );
}

#[no_mangle]
fn reserve_wise() {
    let liquidity_transformer: Key = runtime::get_named_arg("liquidity_transformer");
    let investment_mode: u8 = runtime::get_named_arg("investment_mode");
    let msg_value: U256 = runtime::get_named_arg("msg_value");
    let caller_purse: URef = account::get_main_purse();

    let () = runtime::call_versioned_contract(
        liquidity_transformer.into_hash().unwrap_or_revert().into(),
        None,
        "reserve_wise",
        runtime_args! {
            "investment_mode" => investment_mode,
            "msg_value" => msg_value,
            "caller_purse" => caller_purse
        },
    );
}

#[no_mangle]
fn reserve_wise_with_token() {
    let proxy: Key = runtime::get_named_arg("proxy");
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let investment_mode: u8 = runtime::get_named_arg("investment_mode");
    let caller_purse: URef = account::get_main_purse();

    let () = runtime::call_versioned_contract(
        proxy.into_hash().unwrap_or_revert().into(),
        None,
        "_reserve_wise_with_token",
        runtime_args! {
            "token_address" => token_address,
            "token_amount" => token_amount,
            "investment_mode" => investment_mode,
            "caller_purse" => caller_purse
        },
    );
}

#[no_mangle]
fn _reserve_wise_with_token() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let investment_mode: u8 = runtime::get_named_arg("investment_mode");
    let caller_purse: URef = runtime::get_named_arg("caller_purse");

    let () = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "reserve_wise_with_token",
        runtime_args! {
            "token_address" => token_address,
            "token_amount" => token_amount,
            "investment_mode" => investment_mode,
            "caller_purse" => caller_purse
        },
    );
}

#[no_mangle]
fn forward_liquidity() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());

    let () = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "forward_liquidity",
        runtime_args! {},
    );
}

#[no_mangle]
fn get_my_tokens() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());

    let () = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "get_my_tokens",
        runtime_args! {},
    );
}

#[no_mangle]
fn payout_investor_address() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());

    let investor_address: Key = runtime::get_named_arg("investor_address");

    let ret: U256 = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "payout_investor_address",
        runtime_args! {
            "investor_address" => investor_address
        },
    );

    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn prepare_path() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());

    let token_address: Key = runtime::get_named_arg("token_address");

    let ret: Vec<Key> = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "prepare_path",
        runtime_args! {
            "token_address" => token_address
        },
    );

    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn current_stakeable_day() {
    let liquidity_transformer_address: ContractPackageHash =
        mappings::get_key(&mappings::liquidity_transformer_key());

    let ret: u64 = runtime::call_versioned_contract(
        liquidity_transformer_address,
        None,
        "current_stakeable_day",
        runtime_args! {},
    );

    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn request_refund() {
    let liquidity_transformer: Key = runtime::get_named_arg("liquidity_transformer");
    let proxy_key: Key = runtime::get_named_arg("proxy_key");
    let caller_purse: URef = account::get_main_purse();

    let ret: (U256, U256) = runtime::call_versioned_contract(
        liquidity_transformer.into_hash().unwrap_or_revert().into(),
        None,
        "request_refund",
        runtime_args! {
            "caller_purse" => caller_purse
        },
    );

    let () = runtime::call_versioned_contract(
        proxy_key.into_hash().unwrap_or_revert().into(),
        None,
        "_request_refund",
        runtime_args! {
            "ret" => ret
        },
    );
}

#[no_mangle]
fn _request_refund() {
    let ret: (U256, U256) = runtime::get_named_arg("ret");
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn approve() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let () = runtime::call_versioned_contract(
        token_address.into_hash().unwrap_or_revert().into(),
        None,
        "approve",
        runtime_args! {
            "spender" => spender,
            "amount" => amount
        },
    );
}

#[no_mangle]
fn temp_purse() {
    let liquidity_transformer: Key = runtime::get_named_arg("liquidity_transformer");

    let () = runtime::call_versioned_contract(
        liquidity_transformer.into_hash().unwrap_or_revert().into(),
        None,
        "_temp_purse",
        runtime_args! {
            "purse" => account::get_main_purse()
        },
    );
}

#[no_mangle]
fn _temp_purse() {
    let purse: URef = runtime::get_named_arg("purse");
    mappings::set_key(&mappings::result_key(), purse);
}

#[no_mangle]
fn deposit() {
    let proxy: Key = runtime::get_named_arg("proxy");
    let token: Key = runtime::get_named_arg("token");
    let amount: U512 = runtime::get_named_arg("amount");

    let () = runtime::call_versioned_contract(
        proxy.into_hash().unwrap_or_revert().into(),
        None,
        "_deposit",
        runtime_args! {
            "token" => token,
            "purse" => account::get_main_purse(),
            "amount" => amount,
        },
    );
}

#[no_mangle]
fn _deposit() {
    let token: Key = runtime::get_named_arg("token");
    let purse: URef = runtime::get_named_arg("purse");
    let amount: U512 = runtime::get_named_arg("amount");

    let () = runtime::call_versioned_contract(
        token.into_hash().unwrap_or_revert().into(),
        None,
        "deposit_no_return",
        runtime_args! {
            "purse" => purse,
            "amount" => amount,
        },
    );
}

#[no_mangle]
fn pair_total_supply() {
    let pair: Key = runtime::get_named_arg("pair");

    let ret: U256 = runtime::call_versioned_contract(
        pair.into_hash().unwrap_or_revert().into(),
        None,
        "total_supply",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn increase_cspr() {
    let target: URef = runtime::get_named_arg("target");

    let test_account_cspr_amount: U512 = U512::from(4);
    // (4/*98500000000000 as u128*/).into();

    // let ret = system::transfer_to_account(target, test_account_cspr_amount, None);
    let ret = system::transfer_from_purse_to_purse(
        account::get_main_purse(),
        target,
        test_account_cspr_amount,
        None,
    );

    if ret.is_err() {
        runtime::revert(20);
    }
}

#[no_mangle]
fn transfer_from() {
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let _: Result<(), u32> = runtime::call_versioned_contract(
        wcspr.into_hash().unwrap_or_revert().into(),
        None,
        "transfer_from",
        runtime_args! {
            "owner" => owner,
            "recipient" => recipient,
            "amount" => amount
        },
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractPackageHash::cl_type()),
            Parameter::new("liquidity_transformer", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_settings",
        vec![
            Parameter::new("wise_token", Key::cl_type()),
            Parameter::new("uniswap_pair", Key::cl_type()),
            Parameter::new("synthetic_cspr", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "renounce_keeper",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reserve_wise",
        vec![
            Parameter::new("liquidity_transformer", Key::cl_type()),
            Parameter::new("investment_mode", u8::cl_type()),
            Parameter::new("msg_value", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reserve_wise_with_token",
        vec![
            Parameter::new("proxy", Key::cl_type()),
            Parameter::new("token_address", Key::cl_type()),
            Parameter::new("token_amount", U256::cl_type()),
            Parameter::new("investment_mode", u8::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_reserve_wise_with_token",
        vec![
            Parameter::new("token_address", Key::cl_type()),
            Parameter::new("token_amount", U256::cl_type()),
            Parameter::new("investment_mode", u8::cl_type()),
            Parameter::new("caller_purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "forward_liquidity",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_my_tokens",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "payout_investor_address",
        vec![Parameter::new("investor_address", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "prepare_path",
        vec![Parameter::new("token_address", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "current_stakeable_day",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "request_refund",
        vec![
            Parameter::new("liquidity_transformer", Key::cl_type()),
            Parameter::new("proxy_key", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_request_refund",
        vec![Parameter::new(
            "ret",
            CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("token_address", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "temp_purse",
        vec![Parameter::new("liquidity_transformer", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_temp_purse",
        vec![Parameter::new("purse", URef::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("proxy", Key::cl_type()),
            Parameter::new("amount", U512::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "_deposit",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("purse", URef::cl_type()),
            Parameter::new("amount", U512::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "pair_total_supply",
        vec![Parameter::new("pair", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_cspr",
        vec![Parameter::new("target", URef::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("wcspr", Key::cl_type()),
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let liquidity_transformer: Key = runtime::get_named_arg("liquidity_transformer");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "liquidity_transformer" => liquidity_transformer
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
