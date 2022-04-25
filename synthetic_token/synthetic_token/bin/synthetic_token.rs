#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec};
use casper_contract::{
    contract_api::{account, runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};

use contract_utils::{ContractContext, OnChainContractStorage};
use synthetic_token_crate::erc20_crate::ERC20;
use synthetic_token_crate::synthetic_helper_crate::SYNTHETICHELPER;
use synthetic_token_crate::{self, SYNTHETICTOKEN};

#[derive(Default)]
struct SyntheticToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for SyntheticToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl ERC20<OnChainContractStorage> for SyntheticToken {}
impl SYNTHETICHELPER<OnChainContractStorage> for SyntheticToken {}
impl SYNTHETICTOKEN<OnChainContractStorage> for SyntheticToken {}

impl SyntheticToken {
    fn constructor(
        &mut self,
        wcspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        uniswap_router_package: Key,
        master_address_purse: URef,
    ) {
        SYNTHETICTOKEN::init(
            self,
            wcspr,
            uniswap_pair,
            uniswap_router,
            contract_hash,
            package_hash,
            uniswap_router_package,
            master_address_purse,
        );
    }
}

#[no_mangle]
fn constructor() {
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let uniswap_router_package: Key = runtime::get_named_arg("uniswap_router_package");
    let master_address_purse: URef = runtime::get_named_arg("master_address_purse");

    SyntheticToken::default().constructor(
        wcspr,
        uniswap_pair,
        uniswap_router,
        Key::from(contract_hash),
        package_hash,
        uniswap_router_package,
        master_address_purse,
    );
}

#[no_mangle]
fn get_trading_fee_amount() {
    let previous_evaluation: U256 = runtime::get_named_arg("previous_evaluation");
    let current_evaluation: U256 = runtime::get_named_arg("current_evaluation");

    let ret: U256 =
        SyntheticToken::default().get_trading_fee_amount(previous_evaluation, current_evaluation);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_amount_payout() {
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: U256 = SyntheticToken::default().get_amount_payout(amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_wrapped_balance() {
    let ret: U256 = SyntheticToken::default().get_wrapped_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_synthetic_balance() {
    let ret: U256 = SyntheticToken::default().get_synthetic_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_evaluation() {
    let ret: U256 = SyntheticToken::default().get_evaluation();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_pair_balances() {
    let (ret, _ret): (U256, U256) = SyntheticToken::default().get_pair_balances();
    runtime::ret(CLValue::from_t((ret, _ret)).unwrap_or_revert());
}

#[no_mangle]
fn get_lp_token_balance() {
    let ret: U256 = SyntheticToken::default().get_lp_token_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_liquidity_percent() {
    let ret: U256 = SyntheticToken::default().get_liquidity_percent();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn fees_decision() {
    SyntheticToken::default()._fees_decision();
}

#[no_mangle]
fn extract_and_send_fees() {
    let previous_evaluation: U256 = runtime::get_named_arg("previous_evaluation");
    let current_evaluation: U256 = runtime::get_named_arg("current_evaluation");

    SyntheticToken::default()._extract_and_send_fees(previous_evaluation, current_evaluation);
}

#[no_mangle]
fn swap_exact_tokens_for_tokens() {
    let amount: U256 = runtime::get_named_arg("amount");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let from_token_address: Key = runtime::get_named_arg("from_token_address");
    let to_token_address: Key = runtime::get_named_arg("to_token_address");

    let ret: U256 = SyntheticToken::default()._swap_exact_tokens_for_tokens(
        amount,
        amount_out_min,
        from_token_address,
        to_token_address,
    );
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn add_liquidity() {
    let _amount_wcspr: U256 = runtime::get_named_arg("_amount_wcspr");
    let _amount_scspr: U256 = runtime::get_named_arg("_amount_scspr");

    let (ret, _ret): (U256, U256) =
        SyntheticToken::default()._add_liquidity(_amount_wcspr, _amount_scspr);
    runtime::ret(CLValue::from_t((ret, _ret)).unwrap_or_revert());
}

#[no_mangle]
fn remove_liquidity() {
    let amount: U256 = runtime::get_named_arg("amount");

    let (ret, _ret): (U256, U256) = SyntheticToken::default()._remove_liquidity(amount);
    runtime::ret(CLValue::from_t((ret, _ret)).unwrap_or_revert());
}

#[no_mangle]
fn profit_arbitrage_remove() {
    let ret: U256 = SyntheticToken::default()._profit_arbitrage_remove();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn to_remove_cspr() {
    let ret: U256 = SyntheticToken::default()._to_remove_cspr();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn swap_amount_arbitrage_scspr() {
    let ret: U256 = SyntheticToken::default()._swap_amount_arbitrage_scspr();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn self_burn() {
    SyntheticToken::default()._self_burn();
}

#[no_mangle]
fn clean_up() {
    let deposit_amount: U256 = runtime::get_named_arg("deposit_amount");

    SyntheticToken::default()._clean_up(deposit_amount);
}

#[no_mangle]
fn unwrap() {
    let amount_wcspr: U256 = runtime::get_named_arg("amount_wcspr");

    SyntheticToken::default()._unwrap(amount_wcspr);
}

#[no_mangle]
fn profit() {
    let amount_wcspr: U256 = runtime::get_named_arg("amount_wcspr");

    SyntheticToken::default()._profit(amount_wcspr);
}

#[no_mangle]
fn update_evaluation() {
    SyntheticToken::default()._update_evaluation();
}

#[no_mangle]
fn skim_pair() {
    SyntheticToken::default()._skim_pair();
}

#[no_mangle]
fn arbitrage_decision() {
    SyntheticToken::default()._arbitrage_decision();
}

#[no_mangle]
fn arbitrage_scspr() {
    let wrapped_balance: U256 = runtime::get_named_arg("wrapped_balance");
    let synthetic_balance: U256 = runtime::get_named_arg("synthetic_balance");

    SyntheticToken::default()._arbitrage_scspr(wrapped_balance, synthetic_balance);
}

#[no_mangle]
fn arbitrage_cspr() {
    let wrapped_balance: U256 = runtime::get_named_arg("wrapped_balance");
    let synthetic_balance: U256 = runtime::get_named_arg("synthetic_balance");

    SyntheticToken::default()._arbitrage_cspr(wrapped_balance, synthetic_balance);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("wcspr", Key::cl_type()),
            Parameter::new("uniswap_pair", Key::cl_type()),
            Parameter::new("uniswap_router", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("uniswap_router_package", Key::cl_type()),
            Parameter::new("master_address_purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_trading_fee_amount",
        vec![
            Parameter::new("previous_evaluation", U256::cl_type()),
            Parameter::new("current_evaluation", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amount_payout",
        vec![Parameter::new("amount", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_wrapped_balance",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_synthetic_balance",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_evaluation",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_pair_balances",
        vec![],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_lp_token_balance",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_liquidity_percent",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fees_decision",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "extract_and_send_fees",
        vec![
            Parameter::new("previous_evaluation", U256::cl_type()),
            Parameter::new("current_evaluation", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_exact_tokens_for_tokens",
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("amount_out_min", U256::cl_type()),
            Parameter::new("from_token_address", Key::cl_type()),
            Parameter::new("to_token_address", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_liquidity",
        vec![
            Parameter::new("_amount_wcspr", U256::cl_type()),
            Parameter::new("_amount_scspr", U256::cl_type()),
        ],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "remove_liquidity",
        vec![Parameter::new("amount", U256::cl_type())],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "profit_arbitrage_remove",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "to_remove_cspr",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_amount_arbitrage_scspr",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "self_burn",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "clean_up",
        vec![Parameter::new("deposit_amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "unwrap",
        vec![Parameter::new("amount_wcspr", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "profit",
        vec![Parameter::new("amount_wcspr", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_evaluation",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "skim_pair",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "arbitrage_decision",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "arbitrage_scspr",
        vec![
            Parameter::new("wrapped_balance", U256::cl_type()),
            Parameter::new("synthetic_balance", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "arbitrage_cspr",
        vec![
            Parameter::new("wrapped_balance", U256::cl_type()),
            Parameter::new("synthetic_balance", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let wcspr: Key = runtime::get_named_arg("wcspr");
        let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
        let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
        let uniswap_router_package: Key = runtime::get_named_arg("uniswap_router_package");
        let constructor_args = runtime_args! {
            "wcspr" => wcspr,
            "uniswap_pair" => uniswap_pair,
            "uniswap_router" => uniswap_router,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash,
            "uniswap_router_package" => uniswap_router_package,
            "master_address_purse" => account::get_main_purse()
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
    // If contract package did already exist
    else {
        // get the package
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        // create new version and install it
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
