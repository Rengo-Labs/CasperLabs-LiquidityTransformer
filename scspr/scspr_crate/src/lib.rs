#![no_std]

extern crate alloc;

pub mod data;
pub mod errors;
pub mod event;
pub mod scspr;

pub use scspr::SCSPR;
pub use synthetic_token_crate;
