//! Submodule defining the `Tabular` trait, which assigns a table to a struct.

use std::{hash::Hash, rc::Rc};

/// The `Tabular` trait is used to assign a table to a struct.
pub trait Tabular {
    /// The enumeration of the table names.
    type TableName: std::fmt::Debug + std::fmt::Display + std::marker::Copy + Eq + Hash;

    #[must_use]
    /// The name of the table.
    fn table_name(&self) -> Self::TableName;
}

impl<T: Tabular> Tabular for Rc<T> {
    type TableName = T::TableName;

    fn table_name(&self) -> Self::TableName {
        self.as_ref().table_name()
    }
}

/// The `StaticTabular` trait is used to assign a table to a struct.
pub trait StaticTabular: Tabular {
    #[must_use]
    /// The name of the table.
    fn static_table_name() -> Self::TableName;
}

impl<T: StaticTabular> StaticTabular for Rc<T> {
    fn static_table_name() -> Self::TableName {
        T::static_table_name()
    }
}

/// The `Row` trait is used to assign a table to a struct.
pub trait Row: Tabular + PartialEq + Clone {
    /// The enumeration of the table primary keys.
    type PrimaryKey: std::fmt::Debug
        + std::marker::Copy
        + Eq
        + Hash
        + Ord
        + Tabular<TableName = Self::TableName>;

    #[must_use]
    /// The primary key of the current struct.
    fn primary_key(&self) -> Self::PrimaryKey;
}

impl<R: Row> Row for Rc<R> {
    type PrimaryKey = R::PrimaryKey;

    fn primary_key(&self) -> Self::PrimaryKey {
        self.as_ref().primary_key()
    }
}

/// The `Row` trait is used to assign a table to a struct.
pub trait Rows: Tabular {
    /// The enumeration of the table primary keys.
    type PrimaryKey: std::fmt::Debug
        + std::marker::Copy
        + Eq
        + Hash
        + Tabular<TableName = Self::TableName>;

    #[must_use]
    /// The primary key of the current struct.
    fn primary_keys(&self) -> Vec<Self::PrimaryKey>;
}

impl<R> Tabular for Vec<R>
where
    R: StaticTabular,
{
    type TableName = R::TableName;

    fn table_name(&self) -> Self::TableName {
        Self::static_table_name()
    }
}

impl<R> StaticTabular for Vec<R>
where
    R: StaticTabular,
{
    fn static_table_name() -> Self::TableName {
        R::static_table_name()
    }
}

impl<R: Row + StaticTabular> Rows for Vec<R>
where
    <R as Row>::PrimaryKey: Tabular<TableName = R::TableName>,
{
    type PrimaryKey = R::PrimaryKey;

    fn primary_keys(&self) -> Vec<Self::PrimaryKey> {
        self.iter().map(|row| row.primary_key()).collect()
    }
}
