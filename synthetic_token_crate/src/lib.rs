#![no_std]

extern crate alloc;

pub mod data;
pub mod error;
pub mod event;
pub mod synthetic_token;

pub use casperlabs_erc20;
pub use synthetic_helper_crate;
pub use synthetic_token::SYNTHETICTOKEN;
