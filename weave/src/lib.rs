extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use zdd::*;

pub mod core;
pub mod bdd;
mod zdd;

