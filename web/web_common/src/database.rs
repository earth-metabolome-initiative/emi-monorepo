//! Submodule providing structs relative to the database.
pub mod inserts;
pub mod roles;
pub mod tables;
pub mod updates;
pub mod views;
pub use inserts::Insert;
pub use tables::*;
pub use updates::Update;
pub use views::*;
pub mod operations;
pub use operations::*;
pub mod view_implementations;
pub use view_implementations::*;