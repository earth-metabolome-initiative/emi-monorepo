//! A trait defining a `Read` table entry.

pub mod r#async;
#[cfg(feature = "diesel-async")]
pub use r#async::{AsyncBoundedRead, AsyncRead};
pub mod sync;
pub use sync::{BoundedRead, Read};
pub mod dispatch;
#[cfg(feature = "diesel-async")]
pub use dispatch::{AsyncBoundedReadDispatch, AsyncReadDispatch};
pub use dispatch::{BoundedReadDispatch, ReadDispatch};
