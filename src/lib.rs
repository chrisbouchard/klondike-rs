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
#![feature(const_fn)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

pub mod display;
pub mod engine;
pub mod error;
pub mod model;
pub mod terminal;
mod utils;
