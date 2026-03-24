#[deprecated]
pub mod prelude;

// Defines base test helpers and also re-exports common test macros and utils.
pub use genotype_parser::test::*;

pub use insta::{assert_debug_snapshot, assert_ron_snapshot, assert_snapshot};
pub use pretty_assertions::{
    assert_eq as assert_equal, assert_ne as assert_not_equal, assert_str_eq as assert_str_equal,
};

mod tree;
pub use tree::*;
