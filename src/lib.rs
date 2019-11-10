//! Library to implement the Klondike solitaire game

#![warn(
    clippy::all,
    missing_debug_implementations,
    // TODO: Turn this back on
    // missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_enum;
#[macro_use]
extern crate snafu;

pub mod display;
pub mod engine;
pub mod model;
pub mod terminal;
mod utils;
