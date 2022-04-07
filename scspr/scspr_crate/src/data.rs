use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ContractPackageHash, Key, URef};
use contract_utils::{get_key, set_key};

pub const MAIN_PURSE: &str = "main_purse";

pub const WISE_CONTRACT: &str = "wise_contract";
pub const SYNTHETIC_TOKEN: &str = "synthetic_token";
pub const UNISWAP_FACTORY: &str = "uniswap_factory";

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const OWNER: &str = "owner";

pub fn set_main_purse(purse: URef) {
    set_key(MAIN_PURSE, Key::from(purse));
}

pub fn get_main_purse() -> URef {
    let contract_main_purse_key: Key = get_key(MAIN_PURSE).unwrap_or_revert();
    let contract_main_purse = contract_main_purse_key.as_uref().unwrap_or_revert();
    *contract_main_purse
}

pub fn set_wise_contract(wise_contract: Key) {
    set_key(WISE_CONTRACT, wise_contract);
}

pub fn get_wise_contract() -> Key {
    get_key(WISE_CONTRACT).unwrap_or_revert()
}

pub fn set_synthetic_token(synthetic_token: Key) {
    set_key(SYNTHETIC_TOKEN, synthetic_token);
}

pub fn get_synthetic_token() -> Key {
    get_key(SYNTHETIC_TOKEN).unwrap_or_revert()
}

pub fn set_uniswap_factory(uniswap_factory: Key) {
    set_key(UNISWAP_FACTORY, uniswap_factory);
}

pub fn get_uniswap_factory() -> Key {
    get_key(UNISWAP_FACTORY).unwrap_or_revert()
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_contract_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn get_owner() -> Key {
    get_key(OWNER).unwrap_or_revert()
}
