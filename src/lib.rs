#![warn(
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
/* missing_docs */

#![feature(const_fn)]
#![feature(iter_unfold)]

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;

pub mod display;
pub mod model;
pub mod terminal;
mod utils;
