#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod observation;
/// The prelude module
pub mod prelude {
    pub use crate::observation::*;
}
