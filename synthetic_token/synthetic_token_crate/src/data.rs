use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256};
use contract_utils::{get_key, set_key};

pub const OWNER: &str = "owner";
pub const SELF_PURSE: &str = "self_purse";

pub const CURRENT_EVALUATION: &str = "current_evaluation";
pub const TOKEN_DEFINED: &str = "token_defined";
pub const ALLOW_DEPOSIT: &str = "allow_deposit";
pub const HELPER_DEFINED: &str = "helper_defined";
pub const BYPASS_ENABLED: &str = "bypass_enabled";

pub const WCSPR: &str = "wcspr";
pub const UNISWAP_PAIR: &str = "uniswap_pair";
pub const UNISWAP_ROUTER: &str = "uniswap_router";
pub const TRANSFER_HELPER: &str = "transfer_helper";
pub const MASTER_ADDRESS: &str = "master_address";
pub const MASTER_ADDRESS_PURSE: &str = "master_address_purse";

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

pub const SCSPR: &str = "scspr";

#[repr(u16)]
pub enum ErrorCodes {
    Abort = 35,
}

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

pub fn set_scspr(scspr: Key) {
    set_key(SCSPR, scspr);
}

pub fn scspr() -> Key {
    get_key(SCSPR).unwrap_or(zero_address())
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn owner() -> Key {
    get_key(OWNER).unwrap_or(zero_address())
}

pub fn set_current_evaluation(current_evaluation: U256) {
    set_key(CURRENT_EVALUATION, current_evaluation);
}

pub fn get_current_evaluation() -> U256 {
    get_key(CURRENT_EVALUATION).unwrap_or_default()
}

pub fn set_token_defined(token_defined: bool) {
    set_key(TOKEN_DEFINED, token_defined);
}

pub fn get_token_defined() -> bool {
    get_key(TOKEN_DEFINED).unwrap_or_default()
}

pub fn set_allow_deposit(allow_deposit: bool) {
    set_key(ALLOW_DEPOSIT, allow_deposit);
}

pub fn get_allow_deposit() -> bool {
    get_key(ALLOW_DEPOSIT).unwrap_or_default()
}

pub fn set_helper_defined(helper_defined: bool) {
    set_key(HELPER_DEFINED, helper_defined);
}

pub fn get_helper_defined() -> bool {
    get_key(HELPER_DEFINED).unwrap_or_default()
}

pub fn set_bypass_enabled(bypass_enabled: bool) {
    set_key(BYPASS_ENABLED, bypass_enabled);
}

pub fn get_bypass_enabled() -> bool {
    get_key(BYPASS_ENABLED).unwrap_or_default()
}

pub fn set_wcspr(wcspr: Key) {
    set_key(WCSPR, wcspr);
}

pub fn get_wcspr() -> Key {
    get_key(WCSPR).unwrap_or(zero_address())
}

pub fn set_uniswap_pair(uniswap_pair: Key) {
    set_key(UNISWAP_PAIR, uniswap_pair);
}

pub fn get_uniswap_pair() -> Key {
    get_key(UNISWAP_PAIR).unwrap_or(zero_address())
}

pub fn set_uniswap_router(uniswap_router: Key) {
    set_key(UNISWAP_ROUTER, uniswap_router);
}

pub fn get_uniswap_router() -> Key {
    get_key(UNISWAP_ROUTER).unwrap_or(zero_address())
}

pub fn set_transfer_helper(transfer_helper: Key) {
    set_key(TRANSFER_HELPER, transfer_helper);
}

pub fn get_transfer_helper() -> Key {
    get_key(TRANSFER_HELPER).unwrap_or(zero_address())
}

pub fn set_master_address(master_address: Key) {
    set_key(MASTER_ADDRESS, master_address);
}

pub fn get_master_address() -> Key {
    get_key(MASTER_ADDRESS).unwrap_or(zero_address())
}

pub fn set_master_address_purse(purse: URef) {
    runtime::put_key(&MASTER_ADDRESS_PURSE, purse.into());
}

pub fn get_master_address_purse() -> URef {
    let destination_purse_key = runtime::get_key(&MASTER_ADDRESS_PURSE).unwrap_or_revert();

    match destination_purse_key.as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::User(20)),
    }
}

pub fn set_contract_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_contract_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or(zero_address())
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_default()
}
