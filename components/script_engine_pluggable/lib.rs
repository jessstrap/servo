#![feature(macro_reexport)]

#[cfg(feature = "default_script_engine")]
#[macro_use]
#[macro_reexport(rooted)]
pub extern crate js as inner;

#[cfg(feature = "nojs")]
#[macro_use]
#[macro_reexport(rooted)]
pub extern crate rust_as_script_engine as inner;

pub use inner::*;
