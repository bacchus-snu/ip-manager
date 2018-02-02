#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub mod errors;
pub use errors::{Error, ErrorKind, Result};

pub mod settings;
pub use settings::Settings;

pub mod slack;
