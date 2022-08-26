use casper_types::{ContractPackageHash, Key};
use casperlabs_contract_utils::{get_key, set_key};

pub const WISE_CONTRACT: &str = "wise_contract";
pub const SYNTHETIC_TOKEN: &str = "synthetic_token";
pub const UNISWAP_FACTORY: &str = "uniswap_factory";

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const OWNER: &str = "owner";

pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}

pub fn set_wise_contract(wise_contract: Key) {
    set_key(WISE_CONTRACT, wise_contract);
}

pub fn get_wise_contract() -> Key {
    get_key(WISE_CONTRACT).unwrap_or_else(zero_address)
}

pub fn set_uniswap_factory(uniswap_factory: Key) {
    set_key(UNISWAP_FACTORY, uniswap_factory);
}

pub fn get_uniswap_factory() -> Key {
    get_key(UNISWAP_FACTORY).unwrap_or_else(zero_address)
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_else(zero_address)
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_contract_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn get_owner() -> Key {
    get_key(OWNER).unwrap_or_else(zero_address)
}
