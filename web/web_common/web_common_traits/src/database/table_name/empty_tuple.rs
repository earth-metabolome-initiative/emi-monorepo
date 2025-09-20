//! Submodule implementing traits from the `table_name` module for the
//! `EmptyTuple` type.

use common_traits::builder::EmptyTuple;

use super::TableField;
use crate::database::{HasTableType, table_name::TableEnum};

impl TableEnum for EmptyTuple {}
impl HasTableType for EmptyTuple {
    type Table = EmptyTuple;
}
impl TableField for EmptyTuple {}
