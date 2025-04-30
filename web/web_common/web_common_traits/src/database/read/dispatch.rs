//! Submodule defining the enum-dispatchable variants of the `Read` trait.

pub mod r#async;
pub mod sync;
#[cfg(feature = "diesel-async")]
pub use r#async::{AsyncBoundedReadDispatch, AsyncReadDispatch};
pub use sync::{BoundedReadDispatch, ReadDispatch};
