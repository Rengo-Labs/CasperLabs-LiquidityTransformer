#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{account, runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key,
    Parameter, RuntimeArgs, URef, U256,
};

pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let scspr: Key = runtime::get_named_arg("scspr");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::scspr_key(),
        ContractHash::from(scspr.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn temp_purse() {
    let contract: Key = runtime::get_named_arg("contract");

    let () = runtime::call_contract(
        contract.into_hash().unwrap_or_revert().into(),
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
fn set_master() {
    let scspr_address: ContractHash = mappings::get_key(&mappings::scspr_key());
    let master_address: ContractPackageHash = mappings::get_key(&mappings::self_package_key());

    let () = runtime::call_contract(
        scspr_address,
        "set_master",
        runtime_args! {
            "master_address" => Key::from(master_address)
        },
    );
}

#[no_mangle]
fn forward_ownership() {
    let new_master: Key = runtime::get_named_arg("new_master");
    let scspr_address: ContractHash = mappings::get_key(&mappings::scspr_key());
    let args: RuntimeArgs = runtime_args! {
        "new_master" => new_master
    };
    let () = runtime::call_contract(scspr_address, "forward_ownership", args);
}

#[no_mangle]
fn form_liquidity() {
    let pair: Key = runtime::get_named_arg("pair");
    let purse: URef = runtime::get_named_arg("purse");
    let scspr_address: ContractHash = mappings::get_key(&mappings::scspr_key());

    let ret: U256 = runtime::call_contract(
        scspr_address,
        "form_liquidity",
        runtime_args! {
            "pair" => pair,
            "purse" => purse
        },
    );
    mappings::set_key(&mappings::form_liquidity_key(), ret);
}

#[no_mangle]
fn define_helper() {
    let scspr_address: ContractHash = mappings::get_key(&mappings::scspr_key());
    let transfer_helper: Key = runtime::get_named_arg("transfer_helper");
    let args: RuntimeArgs = runtime_args! {
        "transfer_helper" => transfer_helper
    };
    let ret: Key = runtime::call_contract(scspr_address, "define_helper", args);
    mappings::set_key(&mappings::transfer_helper_key(), ret);
}

#[no_mangle]
fn define_token() {
    let scspr_address: ContractHash = mappings::get_key(&mappings::scspr_key());
    let wise_token: Key = runtime::get_named_arg("wise_token");
    let args: RuntimeArgs = runtime_args! {
        "wise_token" => wise_token
    };
    let ret: Key = runtime::call_contract(scspr_address, "define_token", args);
    mappings::set_key(&mappings::transfer_helper_key(), ret);
}

#[no_mangle]
fn get_token0() {
    let pair: ContractHash = runtime::get_named_arg("pair");
    let args: RuntimeArgs = runtime_args! {};
    let ret: Key = runtime::call_contract(pair, "token0", args);
    mappings::set_key(&mappings::token0_key(), ret);
}

#[no_mangle]
fn get_token1() {
    let pair: ContractHash = runtime::get_named_arg("pair");
    let args: RuntimeArgs = runtime_args! {};
    let ret: Key = runtime::call_contract(pair, "token1", args);
    mappings::set_key(&mappings::token1_key(), ret);
}

#[no_mangle]
fn set_wise() {
    let scspr_address: ContractHash = mappings::get_key(&mappings::scspr_key());

    let wise: Key = runtime::get_named_arg("wise");
    let () = runtime::call_contract(
        scspr_address,
        "set_wise",
        runtime_args! {
            "wise" => wise
        },
    );
}

#[no_mangle]
fn approve() {
    let token: Key = runtime::get_named_arg("token");
    let _spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let package_hash: ContractPackageHash = runtime::call_contract(
        _spender.into_hash().unwrap_or_revert().into(),
        "package_hash",
        runtime_args! {},
    );
    let spender: Key = Key::from(package_hash);

    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount
    };
    let () = runtime::call_contract(token.into_hash().unwrap_or_revert().into(), "approve", args);
}

#[no_mangle]
fn pair_total_supply() {
    let pair: Key = runtime::get_named_arg("pair");

    let ret: U256 = runtime::call_contract(
        pair.into_hash().unwrap_or_revert().into(),
        "total_supply",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn reserve_wise() {
    let liquidity_transformer: Key = runtime::get_named_arg("liquidity_transformer");
    let investment_mode: u8 = runtime::get_named_arg("investment_mode");
    let msg_value: U256 = runtime::get_named_arg("msg_value");
    let caller_purse: URef = account::get_main_purse();

    let () = runtime::call_contract(
        liquidity_transformer.into_hash().unwrap_or_revert().into(),
        "reserve_wise",
        runtime_args! {
            "investment_mode" => investment_mode,
            "msg_value" => msg_value,
            "caller_purse" => caller_purse
        },
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("scspr", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "temp_purse",
        vec![Parameter::new("contract", Key::cl_type())],
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
        "set_master",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_wise",
        vec![Parameter::new("wise", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "forward_ownership",
        vec![Parameter::new("new_master", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "form_liquidity",
        vec![
            Parameter::new("pair", Key::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "define_helper",
        vec![Parameter::new("transfer_helper", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "define_token",
        vec![Parameter::new("wise_token", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_token0",
        vec![Parameter::new("pair", ContractHash::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_token1",
        vec![Parameter::new("pair", ContractHash::cl_type())],
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
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let scspr: Key = runtime::get_named_arg("scspr");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "scspr" => scspr
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
