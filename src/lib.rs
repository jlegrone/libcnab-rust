#![cfg_attr(test, deny(warnings))]
#![warn(rust_2018_idioms)]

// #[macro_use]
// extern crate shrinkwraprs;

mod cnab;
pub use crate::cnab::*;

#[cfg(test)]
mod tests;
