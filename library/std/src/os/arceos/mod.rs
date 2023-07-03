#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
pub mod abi;

#[stable(feature = "rust1", since = "1.0.0")]
pub use abi::*;

pub mod ffi;
#[stable(feature = "rust1", since = "1.0.0")]
pub mod fs;

#[stable(feature = "rust1", since = "1.0.0")]
pub use arceos_api as api;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::fs::FileTypeExt;
}
