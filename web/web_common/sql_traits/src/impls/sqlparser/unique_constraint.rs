//! Implement the [`UniqueConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::UniqueConstraint;

use crate::traits::UniqueIndexLike;

impl UniqueIndexLike for UniqueConstraint {
    fn clause(&self) -> String {
        self.columns.iter().map(|ident| ident.column.to_string()).collect::<Vec<_>>().join(", ")
    }
}
