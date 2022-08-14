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
use contract_utils::{ContractContext, OnChainContractStorage};
use scspr_crate::{
    self,
    synthetic_token_crate::{
        erc20_crate::ERC20, synthetic_helper_crate::SYNTHETICHELPER, SYNTHETICTOKEN,
    },
    SCSPR,
};

#[derive(Default)]
struct Scspr(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Scspr {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl SCSPR<OnChainContractStorage> for Scspr {}
impl SYNTHETICTOKEN<OnChainContractStorage> for Scspr {}
impl SYNTHETICHELPER<OnChainContractStorage> for Scspr {}
impl ERC20<OnChainContractStorage> for Scspr {}

impl Scspr {
    #[allow(clippy::too_many_arguments)]
    fn constructor(
        &mut self,
        wcspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
        uniswap_factory: Key,
        synthetic_token: Key,
        transfer_helper: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        SYNTHETICTOKEN::init(
            self,
            wcspr,
            uniswap_pair,
            uniswap_router,
            transfer_helper,
            Key::from(contract_hash),
            package_hash,
        );
        SCSPR::init(
            self,
            uniswap_factory,
            synthetic_token,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let uniswap_factory: Key = runtime::get_named_arg("uniswap_factory");
    let synthetic_token: Key = runtime::get_named_arg("synthetic_token");
    let transfer_helper: Key = runtime::get_named_arg("transfer_helper");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Scspr::default().constructor(
        wcspr,
        uniswap_pair,
        uniswap_router,
        uniswap_factory,
        synthetic_token,
        transfer_helper,
        contract_hash,
        package_hash,
    );
}

#[no_mangle]
fn set_master() {
    let master_address: Key = runtime::get_named_arg("master_address");

    Scspr::default().set_master(master_address);
}

#[no_mangle]
fn set_wise() {
    let wise: Key = runtime::get_named_arg("wise");
    Scspr::default().set_wise(wise);
}

#[no_mangle]
fn deposit() {
    let msg_value: U256 = runtime::get_named_arg("msg_value");
    let succesor_purse: URef = runtime::get_named_arg("succesor_purse");
    Scspr::default().deposit(msg_value, succesor_purse);
}

#[no_mangle]
fn withdraw() {
    let msg_value: U256 = runtime::get_named_arg("msg_value");
    let succesor_purse: URef = runtime::get_named_arg("succesor_purse");
    Scspr::default().withdraw(msg_value, succesor_purse);
}

#[no_mangle]
fn liquidity_deposit() {
    let msg_value: U256 = runtime::get_named_arg("msg_value");
    Scspr::default().liquidity_deposit(msg_value);
}

#[no_mangle]
fn form_liquidity() {
    let purse: URef = runtime::get_named_arg("purse");
    let pair: Key = runtime::get_named_arg("pair");

    let ret: U256 = Scspr::default().form_liquidity(Some(pair), purse);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn renounce_ownership() {
    Scspr::default().renounce_ownership();
}

#[no_mangle]
fn forward_ownership() {
    let new_master: Key = runtime::get_named_arg("new_master");
    Scspr::default().forward_ownership(new_master);
}

#[no_mangle]
fn define_token() {
    let wise_token: Key = runtime::get_named_arg("wise_token");
    let ret = Scspr::default().define_token(wise_token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn define_helper() {
    let transfer_helper: Key = runtime::get_named_arg("transfer_helper");
    let ret = Scspr::default().define_helper(transfer_helper);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn create_pair() {
    let pair: Key = runtime::get_named_arg("pair");
    Scspr::default().create_pair(pair);
}

#[no_mangle]
fn mint() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    Scspr::default().mint(recipient, amount)
}

#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    Scspr::default().approve(spender, amount);
}

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = Scspr::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");

    let ret: U256 = Scspr::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn fund_contract() {
    let purse: URef = runtime::get_named_arg("purse");
    let amount: U512 = runtime::get_named_arg("amount");

    Scspr::default().fund_contract(purse, amount);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("wcspr", Key::cl_type()),
            Parameter::new("uniswap_pair", Key::cl_type()),
            Parameter::new("uniswap_router", Key::cl_type()),
            Parameter::new("uniswap_factory", Key::cl_type()),
            Parameter::new("synthetic_token", Key::cl_type()),
            Parameter::new("transfer_helper", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_master",
        vec![Parameter::new("master_address", Key::cl_type())],
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
        "deposit",
        vec![
            Parameter::new("msg_value", U256::cl_type()),
            Parameter::new("succesor_purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("msg_value", U256::cl_type()),
            Parameter::new("succesor_purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "liquidity_deposit",
        vec![Parameter::new("msg_value", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "form_liquidity",
        vec![
            Parameter::new("purse", URef::cl_type()),
            Parameter::new("pair", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "renounce_ownership",
        vec![],
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
        "define_token",
        vec![Parameter::new("wise_token", Key::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "define_helper",
        vec![Parameter::new("transfer_helper", Key::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_pair",
        vec![Parameter::new("pair", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
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
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
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
        let uniswap_factory: Key = runtime::get_named_arg("uniswap_factory");
        let synthetic_token: Key = runtime::get_named_arg("synthetic_token");
        let transfer_helper: Key = runtime::get_named_arg("transfer_helper");
        let constructor_args = runtime_args! {
            "wcspr" => wcspr,
            "uniswap_pair" => uniswap_pair,
            "uniswap_router" => uniswap_router,
            "uniswap_factory" => uniswap_factory,
            "synthetic_token" => synthetic_token,
            "transfer_helper" => transfer_helper,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash
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
