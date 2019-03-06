extern crate itertools;
#[cfg(test)]
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate serde_derive;
extern crate weave;

pub use closet::*;
pub use closet_builder::*;
pub use core::*;

mod closet_builder;
mod core;
mod closet;

