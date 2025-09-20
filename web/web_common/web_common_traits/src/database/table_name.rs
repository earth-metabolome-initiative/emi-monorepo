//! Submodule providing the `TableName` trait for struct tables.

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
mod empty_tuple;
use common_traits::builder::Attributed;

/// Trait defining the type of a table enumeration.
pub trait HasTableType {
    /// The type of the table enumeration.
    type Table: TableEnum;
}

/// Trait defining a table enumeration.
pub trait TableEnum: FromStr + Debug + Display {}

/// Trait defining a field enumeration for a table.
pub trait TableField: FromStr + HasTableType + Debug + Display {}

/// Trait defining that a type has an associated table attribute enumeration.
pub trait TableAttributed:
    Attributed<Attribute = <Self as TableAttributed>::TableAttribute>
{
    /// The type of the table attribute enumeration.
    type TableAttribute: TableField;
}

impl<T> TableAttributed for T
where
    T: Attributed,
    T::Attribute: TableField,
{
    type TableAttribute = T::Attribute;
}

/// Trait defining the name of a table in the database.
pub trait TableName {
    /// Constant representing the name of the table.
    const TABLE_NAME: &'static str;
}
