use alloc::string::ToString;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ApiError, CLTyped, Key, URef, U256,
};
use casperlabs_contract_utils::{get_key, key_to_str, set_key, Dict};

pub const WISE_CONTRACT: &str = "wise_contract";
pub const PAIR_WISE: &str = "pair_wise";
pub const PAIR_SCSPR: &str = "pair_scspr";
pub const UNISWAP_ROUTER: &str = "uniswap_router";
pub const UNISWAP_ROUTER_PACKAGE: &str = "uniswap_router_package";
pub const WCSPR: &str = "wcspr";
pub const SCSPR: &str = "scspr";

pub const INVESTMENT_DAYS: u8 = 15;
pub const MAX_SUPPLY: u128 = 264_000_000_000_000_000; // 264000000E9;
pub const MAX_INVEST: u128 = 200_000_000_000_000; // 200000E9;
pub const TOKEN_COST: u128 = MAX_INVEST / (MAX_SUPPLY / 1_000_000_000); // MAX_INVEST / (MAX_SUPPLY / 1E9);
pub const REFUND_CAP: u128 = 100_000_000_000; // 100E9;

pub const UNIQUE_INVESTORS: &str = "unique_investors";
pub const PURCHASED_TOKENS: &str = "purchased_tokens";
pub const INVESTOR_BALANCE: &str = "investor_balance";

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const SELF_PURSE: &str = "self_purse";

pub const SETTINGS_KEEPER: &str = "settings_keeper";

pub const GLOBALS: &str = "globals";

pub const CASH_BACK_TOTAL: &str = "cash_back_total";
pub const INVESTOR_COUNT: &str = "investor_count";
pub const TOTAL_TRANSFER_TOKENS: &str = "total_transfer_tokens";
pub const TOTAL_CSPR_CONTRIBUTED: &str = "total_cspr_contributed";
pub const UNISWAP_SWAPED: &str = "uniswap_swaped";

pub struct Globals {
    dict: Dict,
}

impl Globals {
    pub fn instance() -> Globals {
        Globals {
            dict: Dict::instance(GLOBALS),
        }
    }

    pub fn init() {
        Dict::init(GLOBALS)
    }

    pub fn get<T: FromBytes + CLTyped + Default>(&self, owner: &str) -> T {
        self.dict.get(owner).unwrap_or_default()
    }

    pub fn set<T: ToBytes + CLTyped>(&self, owner: &str, value: T) {
        self.dict.set(owner, value);
    }
}

pub struct InvestorBalance {
    dict: Dict,
}

impl InvestorBalance {
    pub fn instance() -> InvestorBalance {
        InvestorBalance {
            dict: Dict::instance(INVESTOR_BALANCE),
        }
    }

    pub fn init() {
        Dict::init(INVESTOR_BALANCE)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get(&key_to_str(key)).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set(&key_to_str(key), value);
    }
}

pub struct PurchasedTokens {
    dict: Dict,
}

impl PurchasedTokens {
    pub fn instance() -> PurchasedTokens {
        PurchasedTokens {
            dict: Dict::instance(PURCHASED_TOKENS),
        }
    }

    pub fn init() {
        Dict::init(PURCHASED_TOKENS)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get(&key_to_str(key)).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set(&key_to_str(key), value);
    }
}

pub struct UniqueInvestors {
    dict: Dict,
}

impl UniqueInvestors {
    pub fn instance() -> UniqueInvestors {
        UniqueInvestors {
            dict: Dict::instance(UNIQUE_INVESTORS),
        }
    }

    pub fn init() {
        Dict::init(UNIQUE_INVESTORS)
    }

    pub fn get(&self, owner: &U256) -> Key {
        self.dict.get(owner.to_string().as_str()).unwrap()
    }

    pub fn set(&self, owner: &U256, value: Key) {
        self.dict.set(owner.to_string().as_str(), value);
    }
}

pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}

pub fn uniswap_router() -> Key {
    get_key(UNISWAP_ROUTER).unwrap_or_else(zero_address)
}

pub fn set_uniswap_router(uniswap_router: Key) {
    set_key(UNISWAP_ROUTER, uniswap_router);
}

pub fn wcspr() -> Key {
    get_key(WCSPR).unwrap_or_else(zero_address)
}

pub fn set_wcspr(wcspr: Key) {
    set_key(WCSPR, wcspr);
}

pub fn scspr() -> Key {
    get_key(SCSPR).unwrap_or_else(zero_address)
}

pub fn set_scspr(wcspr: Key) {
    set_key(SCSPR, wcspr);
}

pub fn hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_else(zero_address)
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn package() -> Key {
    get_key(SELF_PACKAGE_HASH).unwrap_or_else(zero_address)
}

pub fn set_package(package_hash: Key) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn wise() -> Key {
    get_key(WISE_CONTRACT).unwrap_or_else(zero_address)
}

pub fn set_wise(hash: Key) {
    set_key(WISE_CONTRACT, hash);
}

pub fn pair_wise() -> Key {
    get_key(PAIR_WISE).unwrap_or_else(zero_address)
}

pub fn set_pair_wise(pair: Key) {
    set_key(PAIR_WISE, pair);
}

pub fn pair_scspr() -> Key {
    get_key(PAIR_SCSPR).unwrap_or_else(zero_address)
}

pub fn set_pair_scspr(pair: Key) {
    set_key(PAIR_SCSPR, pair);
}

pub fn settings_keeper() -> Key {
    get_key(SETTINGS_KEEPER).unwrap_or_else(zero_address)
}

pub fn set_settings_keeper(hash: Key) {
    set_key(SETTINGS_KEEPER, hash);
}

pub fn self_purse() -> URef {
    let destination_purse_key = runtime::get_key(SELF_PURSE).unwrap_or_revert();
    match destination_purse_key.as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::User(12)),
    }
}

pub fn set_self_purse(purse: URef) {
    runtime::put_key(SELF_PURSE, purse.into());
}
