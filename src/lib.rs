//! Library to implement the Klondike solitaire game

#![warn(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![feature(const_fn, try_trait)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate log;
#[macro_use]
extern crate num_enum;
#[macro_use]
extern crate snafu;

pub mod display;
pub mod engine;
pub mod error;
pub mod model;
pub mod terminal;
mod utils;
