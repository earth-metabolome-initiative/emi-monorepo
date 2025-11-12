//! Submodule for objects whose main metadata is just that they are part of a
//! table.

use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A struct associating a table with one of its attributes, such as a column or
/// constraint.
pub struct TableAttribute<T, A> {
    /// The attribute associated with the table.
    attribute: A,
    /// The table the attribute belongs to.
    table: Rc<T>,
}

impl<T, A> Display for TableAttribute<T, A>
where
    A: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.attribute)
    }
}

impl<T, A> TableAttribute<T, A> {
    /// Creates a new `TableAttribute` associating the given table with the
    /// given attribute.
    #[inline]
    pub fn new(table: Rc<T>, attribute: A) -> Self {
        Self { attribute, table }
    }

    /// Returns a reference to the table.
    #[inline]
    pub fn table(&self) -> &T {
        &self.table
    }

    /// Returns a reference to the attribute.
    #[inline]
    pub fn attribute(&self) -> &A {
        &self.attribute
    }
}
