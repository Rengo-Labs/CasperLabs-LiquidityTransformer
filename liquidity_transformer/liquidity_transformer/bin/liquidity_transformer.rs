#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec};
use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, AccessRights, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs,
    URef, U256, U512,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use liquidity_transformer_crate::{self, data, LIQUIDITYTRANSFORMER};

#[derive(Default)]
struct LiquidityTransformer(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidityTransformer {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDITYTRANSFORMER<OnChainContractStorage> for LiquidityTransformer {}

impl LiquidityTransformer {
    #[allow(clippy::too_many_arguments)]
    fn constructor(
        &mut self,
        wise_token: Key,
        scspr: Key,
        uniswap_pair: Key,
        uniswap_router: Key,
        wcspr: Key,
        package_hash: Key,
        contract_hash: Key,
        purse: URef,
    ) {
        LIQUIDITYTRANSFORMER::init(
            self,
            wise_token,
            scspr,
            uniswap_pair,
            uniswap_router,
            wcspr,
            package_hash,
            contract_hash,
            purse,
        );
    }
}

#[no_mangle]
fn constructor() {
    let wise_token: Key = runtime::get_named_arg("wise_token");
    let scspr: Key = runtime::get_named_arg("scspr");
    let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let purse: URef = runtime::get_named_arg("purse");

    LiquidityTransformer::default().constructor(
        wise_token,
        scspr,
        uniswap_pair,
        uniswap_router,
        wcspr,
        Key::from(package_hash),
        Key::from(contract_hash),
        purse,
    );
}

/// @dev Used to initialize WISE_TOKEN, PAIR and SCSPR contract addresses
#[no_mangle]
fn set_settings() {
    let wise_token: Key = runtime::get_named_arg("wise_token");
    let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
    let synthetic_cspr: Key = runtime::get_named_arg("synthetic_cspr");

    LiquidityTransformer::default().set_settings(wise_token, uniswap_pair, synthetic_cspr);
}

/// @notice Use to renounce_keeper and can be only called by keeper
/// @dev Sets settings_keeper to zero address
#[no_mangle]
fn renounce_keeper() {
    LiquidityTransformer::default().renounce_keeper();
}

/// @dev Performs reservation of WISE tokens with CSPR
#[no_mangle]
fn reserve_wise() {
    let investment_mode: u8 = runtime::get_named_arg("investment_mode");
    let msg_value: U256 = runtime::get_named_arg("msg_value");
    let caller_purse: URef = runtime::get_named_arg("caller_purse");

    LiquidityTransformer::default().reserve_wise(investment_mode, msg_value, caller_purse);
}

/// @notice Allows reservation of WISE tokens with other ERC20 tokens
/// @dev this will require LT contract to be approved as spender
/// @param token_address address of an ERC20 token to use
/// @param token_amount amount of tokens to use for reservation
#[no_mangle]
fn reserve_wise_with_token() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let investment_mode: u8 = runtime::get_named_arg("investment_mode");
    let caller_purse: URef = runtime::get_named_arg("caller_purse");

    LiquidityTransformer::default().reserve_wise_with_token(
        token_address,
        token_amount,
        investment_mode,
        caller_purse,
    );
}

/// @notice Creates initial liquidity on uniswap by forwarding
///     reserved tokens equivalent to CSPR contributed to the contract
/// @dev check add_liquidity documentation
#[no_mangle]
fn forward_liquidity() {
    let pair: Key = runtime::get_named_arg("pair");

    LiquidityTransformer::default().forward_liquidity(pair);
}

/// @notice Allows to mint all the tokens
///     from investor and referrer perspectives
/// @dev can be called after forward_liquidity()
#[no_mangle]
fn get_my_tokens() {
    LiquidityTransformer::default().get_my_tokens();
}

/// @notice Allows to mint tokens for specific investor address
/// @dev aggregades investors tokens across all investment days
///     and uses STAKEABLE_CONTRACT instance to mint all the WISE tokens
/// @param investor_address requested investor calculation address
/// @return _payout amount minted to the investors address
#[no_mangle]
fn payout_investor_address() {
    let investor_address: Key = runtime::get_named_arg("investor_address");

    let ret: U256 = LiquidityTransformer::default().payout_investor_address(investor_address);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Prepares path variable for uniswap to exchange tokens
/// @dev used in reserve_wise_with_token() swap_exact_tokens_for_tokens call
/// @param token_address ERC20 token address to be swapped for CSPR
/// @return _path that is used to swap tokens for CSPR on uniswap
#[no_mangle]
fn prepare_path() {
    let token_address: Key = runtime::get_named_arg("token_address");

    let ret: Vec<Key> = LiquidityTransformer::default().prepare_path(token_address);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Shows current day of WiseToken
/// @dev value is fetched from STAKEABLE_CONTRACT
/// @return iteration day since WISE inception
#[no_mangle]
fn current_stakeable_day() {
    let ret: u64 = LiquidityTransformer::default().current_stakeable_day();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// @notice Allows refunds if funds are stuck
#[no_mangle]
fn request_refund() {
    let caller_purse: URef = runtime::get_named_arg("caller_purse");

    let ret: (U256, U256) = LiquidityTransformer::default().request_refund(caller_purse);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Used for sending funds to contract
/// @dev used as a fallback function
#[no_mangle]
fn fund_contract() {
    let purse: URef = runtime::get_named_arg("purse");
    let amount: U512 = runtime::get_named_arg("amount");

    LiquidityTransformer::default().fund_contract(purse, amount);
}

/// @dev used to fetch purse of contract for sending funds
#[no_mangle]
fn contract_read_only_purse() {
    runtime::ret(
        CLValue::from_t(data::self_purse().with_access_rights(AccessRights::READ_ADD))
            .unwrap_or_revert(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("wise_token", Key::cl_type()),
            Parameter::new("scspr", Key::cl_type()),
            Parameter::new("uniswap_pair", Key::cl_type()),
            Parameter::new("uniswap_router", Key::cl_type()),
            Parameter::new("wcspr", Key::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("purse", URef::cl_type()),
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
            Parameter::new("investment_mode", u8::cl_type()),
            Parameter::new("msg_value", U256::cl_type()),
            Parameter::new("caller_purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reserve_wise_with_token",
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
        vec![Parameter::new("pair", Key::cl_type())],
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
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "prepare_path",
        vec![Parameter::new("token_address", Key::cl_type())],
        CLType::List(Box::new(Key::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "current_stakeable_day",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "request_refund",
        vec![Parameter::new("caller_purse", URef::cl_type())],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fund_contract",
        vec![
            Parameter::new("purse", URef::cl_type()),
            Parameter::new("amount", U512::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "contract_read_only_purse",
        vec![],
        URef::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
pub extern "C" fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // Payable
        let caller_purse = account::get_main_purse();
        let purse: URef = system::create_purse();
        let amount: U512 = runtime::get_named_arg("amount");
        if amount != 0.into() {
            system::transfer_from_purse_to_purse(caller_purse, purse, amount, None)
                .unwrap_or_revert();
        }

        let wise_token: Key = runtime::get_named_arg("wise_token");
        let scspr: Key = runtime::get_named_arg("scspr");
        let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
        let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
        let wcspr: Key = runtime::get_named_arg("wcspr");
        let constructor_args = runtime_args! {
            "wise_token" => wise_token,
            "scspr" => scspr,
            "uniswap_pair" => uniswap_pair,
            "uniswap_router" => uniswap_router,
            "wcspr" => wcspr,
            "package_hash" => package_hash,
            "contract_hash" => contract_hash,
            "purse" => purse
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
