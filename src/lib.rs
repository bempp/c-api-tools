//! C API tools
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

mod types;

pub use c_api_tools_macros::cfuncs;
pub use c_api_tools_macros::concretise_types;

pub use types::DType;
pub use types::DTypeIdentifier;
