#![doc = include_str!("../README.md")]

mod impls;
pub mod traits;

/// Prelude module re-exporting commonly used items from the crate.
pub mod prelude {
    #[cfg(feature = "sqlparser")]
    pub use crate::impls::SqlParserDatabase;
    pub use crate::traits::*;
}
