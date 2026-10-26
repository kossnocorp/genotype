//! Source-preserving Genotype formatting using a native document model and printer.
//! The library performs no filesystem access and supports `wasm32-unknown-unknown`.

pub mod prelude;

mod config;
pub use config::*;

mod doc;
pub use doc::*;

mod error;
pub use error::*;

mod format;
pub use format::*;

mod printer;
