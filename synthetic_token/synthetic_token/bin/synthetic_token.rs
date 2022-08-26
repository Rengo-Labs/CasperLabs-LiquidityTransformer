#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
    U512,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use synthetic_token_crate::{
    self, data, erc20_crate::ERC20, synthetic_helper_crate::SYNTHETICHELPER, SYNTHETICTOKEN,
};

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
    ) {
        SYNTHETICTOKEN::init(
            self,
            wcspr,
            uniswap_pair,
            uniswap_router,
            contract_hash,
            package_hash,
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

    SyntheticToken::default().constructor(
        wcspr,
        uniswap_pair,
        uniswap_router,
        Key::from(contract_hash),
        package_hash,
    );
}

#[no_mangle]
fn uniswap_router() {
    runtime::ret(CLValue::from_t(data::get_uniswap_router()).unwrap_or_revert());
}

#[no_mangle]
fn wcspr() {
    runtime::ret(CLValue::from_t(data::get_wcspr()).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");

    let ret: U256 = SyntheticToken::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
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
fn fund_contract() {
    let purse: URef = runtime::get_named_arg("purse");
    let amount: U512 = runtime::get_named_arg("amount");

    SyntheticToken::default().fund_contract(purse, amount);
}

#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    SyntheticToken::default().approve(spender, amount);
}

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = SyntheticToken::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
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
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "uniswap_router",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "wcspr",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
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
        "fund_contract",
        vec![
            Parameter::new("purse", URef::cl_type()),
            Parameter::new("amount", U512::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
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
        let constructor_args = runtime_args! {
            "wcspr" => wcspr,
            "uniswap_pair" => uniswap_pair,
            "uniswap_router" => uniswap_router,
            "contract_hash" => contract_hash,
            "package_hash" => package_hash,
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
