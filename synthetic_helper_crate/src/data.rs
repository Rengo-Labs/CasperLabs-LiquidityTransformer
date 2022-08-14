use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, URef, U256};

pub const DECIMALS: U256 = U256([18, 0, 0, 0]); // 18
pub const LIMIT_AMOUNT: U256 = U256([13070572018536022016, 10549268516463523069, 293873587705, 0]); // 10 ^ 50

pub const TRADING_FEE: U256 = U256([997500000000, 0, 0, 0]); // 997500000000
pub const TRADING_FEE_QUOTIENT: U256 = U256([1002506265664, 0, 0, 0]); // 1002506265664

pub const EQUALIZE_SIZE_VALUE: U256 = U256([100000000, 0, 0, 0]); // 100000000
pub const ARBITRAGE_CONDITION: U256 = U256([1000001, 0, 0, 0]); // 1000001
pub const TRADING_FEE_CONDITION: U256 = U256([100000001, 0, 0, 0]); // 100000001
pub const LIQUIDITY_PERCENTAGE_CORRECTION: U256 = U256([995000, 0, 0, 0]); // 995000

pub const PRECISION_POINTS: U256 = U256([1000000, 0, 0, 0]); // 1000000
pub const PRECISION_POINTS_POWER2: U256 = U256([1000000000000, 0, 0, 0]); // PRECISION_POINTS * PRECISION_POINTS
pub const PRECISION_POINTS_POWER3: U256 = U256([1000000000000000000, 0, 0, 0]); // PRECISION_POINTS_POWER2 * PRECISION_POINTS
pub const PRECISION_POINTS_POWER4: U256 = U256([2003764205206896640, 54210, 0, 0]); // PRECISION_POINTS_POWER3 * PRECISION_POINTS
pub const PRECISION_POINTS_POWER5: U256 = U256([5076944270305263616, 54210108624, 0, 0]); // PRECISION_POINTS_POWER4 * PRECISION_POINTS

pub const PRECISION_DIFF: U256 = U256([2500000000, 0, 0, 0]); // PRECISION_POINTS_POWER2 - TRADING_FEE;
pub const PRECISION_PROD: U256 = U256([10760958229705916416, 54074, 0, 0]); // PRECISION_POINTS_POWER2 * TRADING_FEE;

pub const PRECISION_FEES_PROD: U256 = U256([997493734335680000, 0, 0, 0]); // TRADING_FEE_QUOTIENT * LIQUIDITY_PERCENTAGE_CORRECTION;

pub const SELF_PURSE: &str = "self_purse";

pub fn set_contract_purse(purse: URef) {
    runtime::put_key(SELF_PURSE, purse.into());
}

pub fn get_contract_purse() -> URef {
    let destination_purse_key = runtime::get_key(SELF_PURSE).unwrap_or_revert();

    match destination_purse_key.as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::User(20)),
    }
}
