#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec};
use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
    U512,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use scspr_crate::{
    self,
    synthetic_token_crate::{
        casperlabs_erc20::ERC20,
        data::{self, get_uniswap_pair, get_uniswap_router, get_wcspr},
        synthetic_helper_crate::SYNTHETICHELPER,
        SYNTHETICTOKEN,
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
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        purse: URef,
    ) {
        SYNTHETICTOKEN::init(
            self,
            wcspr,
            uniswap_pair,
            uniswap_router,
            Key::from(contract_hash),
            package_hash,
        );
        SCSPR::init(
            self,
            uniswap_factory,
            Key::from(contract_hash),
            package_hash,
            purse,
        );
    }
}

#[no_mangle]
fn constructor() {
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
    let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
    let uniswap_factory: Key = runtime::get_named_arg("uniswap_factory");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let purse: URef = runtime::get_named_arg("purse");
    Scspr::default().constructor(
        wcspr,
        uniswap_pair,
        uniswap_router,
        uniswap_factory,
        contract_hash,
        package_hash,
        purse,
    );
}

/// @notice Use to set master address value
#[no_mangle]
fn set_master() {
    let master_address: Key = runtime::get_named_arg("master_address");

    Scspr::default().set_master(master_address);
}

/// @notice Use to set wise address value
#[no_mangle]
fn set_wise() {
    let wise: Key = runtime::get_named_arg("wise");
    Scspr::default().set_wise(wise);
}

/// @notice Use to deposit amount into the scspr contract
/// @param 'amount' deposit amount value
/// @param 'purse' caller purse to deposit value from
#[no_mangle]
fn deposit() {
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");
    Scspr::default().deposit(amount, purse);
}

/// @notice Use to withdraw amount from the scspr withdraw
/// @param 'amount' withdraw amount value
/// @param 'purse' caller purse to withdraw value into
#[no_mangle]
fn withdraw() {
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");
    Scspr::default().withdraw(amount, purse);
}

/// @notice Use to mint tokens to the caller address
/// @param 'amount' mint amount value
/// @param 'purse' caller purse to deposit value from
#[no_mangle]
fn liquidity_deposit() {
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");
    Scspr::default().liquidity_deposit(purse, amount);
}

/// @notice Creates initial liquidity on uniswap by forwarding
///     reserved tokens equivalent to CSPR contributed to the contract
/// @dev check add_liquidity documentation
#[no_mangle]
fn form_liquidity() {
    let ret: U256 = Scspr::default().form_liquidity();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Used to renounce ownership to zero address
#[no_mangle]
fn renounce_ownership() {
    Scspr::default().renounce_ownership();
}

/// @notice Used to forward ownership to new_master
/// @param 'new_master' address to forward ownership
#[no_mangle]
fn forward_ownership() {
    let new_master: Key = runtime::get_named_arg("new_master");
    Scspr::default().forward_ownership(new_master);
}

/// @notice Used to deposit value from contract
///     and transfer pair amount to this contract address
/// @param 'purse' caller purse to deposit value into contract purse
/// @param 'amount' value to be deposited
/// @param 'token_amount' amount of pair tokens to be transfer into contract
#[no_mangle]
fn add_lp_tokens() {
    let purse: URef = runtime::get_named_arg("purse");
    let amount: U256 = runtime::get_named_arg("amount");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    Scspr::default().add_lp_tokens(purse, amount, token_amount);
}

/// @dev used to define token and make set_token_defined true
/// @param 'wise_token' address to call get_synthetic_token_address
///     and check with this scspr token
#[no_mangle]
fn define_token() {
    let wise_token: Key = runtime::get_named_arg("wise_token");
    let ret = Scspr::default().define_token(wise_token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @dev used to define transfer helper and make set_helper_defined true
/// @param 'transfer_helper' address to call get_transfer_invoker_address
///     and check with this scspr token that either it is the tansfer_invoker or not
#[no_mangle]
fn define_helper() {
    let transfer_helper: Key = runtime::get_named_arg("transfer_helper");
    let ret = Scspr::default().define_helper(transfer_helper);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @dev used to create_pair by calling factory create pair
///     and make wcspr & scspr pair
/// @param 'pair' address to make pair on
#[no_mangle]
fn create_pair() {
    Scspr::default().create_pair();
}

/// @dev This function is to mint token against the address that user provided
/// @param `to` A Key that holds the account address of the user
/// @param `amount` A U256 that holds the amount for mint
#[no_mangle]
fn mint() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    Scspr::default().mint(recipient, amount)
}

/// @notice This function is to approve tokens against the address that user provided
/// @param `spender` A Key that holds the account address of the user
/// @param `amount` A U256 that holds the amount for approve
#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    Scspr::default().approve(spender, amount);
}

/// @notice This function is to transfer tokens against the address that user provided
/// @param `recipient` A Key that holds the account address of the user
/// @param `amount` A U256 that holds the amount for transfer
#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = Scspr::default().transfer(recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice This function is to transfer tokens against the address that has been approved before by owner
/// @param `owner` A Key that holds the account address of the user
/// @param `recipient` A Key that holds the account address of the user
/// @param `amount` A U256 that holds the amount for transfer
#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = Scspr::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice This function is to return the Balance  of owner against the address that user provided
/// @param `owner` A Key that holds the account address of the user against which user wants to get balance
#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Scspr::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @notice Used for sending funds to contract
/// @dev used as a fallback function
#[no_mangle]
fn fund_contract() {
    let purse: URef = runtime::get_named_arg("purse");
    let amount: U512 = runtime::get_named_arg("amount");

    Scspr::default().fund_contract(purse, amount);
}

// Synthetic token entrypoints

#[no_mangle]
fn wcspr() {
    runtime::ret(CLValue::from_t(get_wcspr()).unwrap_or_revert());
}

#[no_mangle]
fn uniswap_router() {
    runtime::ret(CLValue::from_t(get_uniswap_router()).unwrap_or_revert());
}

#[no_mangle]
fn uniswap_pair() {
    runtime::ret(CLValue::from_t(get_uniswap_pair()).unwrap_or_revert());
}

#[no_mangle]
fn get_trading_fee_amount() {
    let previous_evaluation: U256 = runtime::get_named_arg("previous_evaluation");
    let current_evaluation: U256 = runtime::get_named_arg("current_evaluation");
    let ret: U256 =
        Scspr::default().get_trading_fee_amount(previous_evaluation, current_evaluation);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_amount_payout() {
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: U256 = Scspr::default().get_amount_payout(amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_synthetic_balance() {
    let ret: U256 = Scspr::default().get_synthetic_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_wrapped_balance() {
    let ret: U256 = Scspr::default().get_wrapped_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_evaluation() {
    let ret: U256 = Scspr::default().get_evaluation();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_pair_balances() {
    let (ret, _ret): (U256, U256) = Scspr::default().get_pair_balances();
    runtime::ret(CLValue::from_t((ret, _ret)).unwrap_or_revert());
}

#[no_mangle]
fn get_lp_token_balance() {
    let ret: U256 = Scspr::default().get_lp_token_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_liquidity_percent() {
    let ret: U256 = Scspr::default().get_liquidity_percent();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn master_address() {
    runtime::ret(CLValue::from_t(data::get_master_address()).unwrap_or_revert());
}

#[no_mangle]
fn current_evaluation() {
    runtime::ret(CLValue::from_t(data::get_current_evaluation()).unwrap_or_revert());
}

#[no_mangle]
fn transfer_helper() {
    runtime::ret(CLValue::from_t(data::get_transfer_helper()).unwrap_or_revert());
}

#[no_mangle]
fn token_defined() {
    runtime::ret(CLValue::from_t(data::get_token_defined()).unwrap_or_revert());
}

#[no_mangle]
fn allow_deposit() {
    runtime::ret(CLValue::from_t(data::get_allow_deposit()).unwrap_or_revert());
}

#[no_mangle]
fn helper_defined() {
    runtime::ret(CLValue::from_t(data::get_helper_defined()).unwrap_or_revert());
}

#[no_mangle]
fn bypass_enabled() {
    runtime::ret(CLValue::from_t(data::get_bypass_enabled()).unwrap_or_revert());
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
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("purse", URef::cl_type()),
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
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "liquidity_deposit",
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "form_liquidity",
        vec![],
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
        "add_lp_tokens",
        vec![
            Parameter::new("purse", URef::cl_type()),
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("token_amount", U256::cl_type()),
        ],
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
        vec![],
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
        "transfer",
        vec![
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
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    // Synthetic token
    entry_points.add_entry_point(EntryPoint::new(
        "wcspr",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
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
        "uniswap_pair",
        vec![],
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
        "get_synthetic_balance",
        vec![],
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

    // Synthetic token

    entry_points.add_entry_point(EntryPoint::new(
        "master_address",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "current_evaluation",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_helper",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token_defined",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allow_deposit",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "helper_defined",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "bypass_enabled",
        vec![],
        bool::cl_type(),
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

        let wcspr: Key = runtime::get_named_arg("wcspr");
        let uniswap_pair: Key = runtime::get_named_arg("uniswap_pair");
        let uniswap_router: Key = runtime::get_named_arg("uniswap_router");
        let uniswap_factory: Key = runtime::get_named_arg("uniswap_factory");
        let constructor_args = runtime_args! {
            "wcspr" => wcspr,
            "uniswap_pair" => uniswap_pair,
            "uniswap_router" => uniswap_router,
            "uniswap_factory" => uniswap_factory,
            "contract_hash" => contract_hash,
            "package_hash" => package_hash,
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
