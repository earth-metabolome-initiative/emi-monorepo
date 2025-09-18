//! Submodule defining the `TableLike` trait and its implementations.

use sqlparser::ast::DataType;

use crate::traits::ColumnLike;

/// Trait representing entities that are "table-like", such as tables and views.
pub trait TableLike {
    /// The type representing a column in the table-like entity.
    type Column: ColumnLike;

    /// Returns the columns which compose the primary key of the table-like
    /// entity, if it has one.
    fn primary_key_columns(&self) -> Option<Vec<&Self::Column>>;

    /// Returns the type of the primary key of the table-like entity, if it has
    /// one.
    fn primary_key_type(&self) -> Option<Vec<&DataType>> {
        let pk_columns = self.primary_key_columns()?;
        Some(pk_columns.iter().map(|col| col.data_type()).collect())
    }

    /// Returns whether the table-like entity has a primary key of type `UUID`.
    fn has_uuid_pk(&self) -> bool {
        match self.primary_key_type().as_deref() {
            Some([data_type]) => data_type == &&DataType::Uuid,
            _ => false,
        }
    }
}

impl TableLike for sqlparser::ast::CreateTable {
    type Column = sqlparser::ast::ColumnDef;

    fn primary_key_columns(&self) -> Option<Vec<&Self::Column>> {
        if let Some(pk_constraint) = self.constraints.iter().find_map(|constraint| {
            if let sqlparser::ast::TableConstraint::PrimaryKey { columns, .. } = constraint {
                Some(columns)
            } else {
                None
            }
        }) {
            return Some(
                pk_constraint
                    .iter()
                    .map(|ident| {
                        self.columns
                            .iter()
                            .find(|col| {
                                col.name.value.eq_ignore_ascii_case(&ident.column.to_string())
                            })
                            .expect(&format!(
                                "Primary key column `{ident}` not found in table columns"
                            ))
                    })
                    .collect::<Vec<_>>(),
            );
        }

        for column in &self.columns {
            if column.is_primary_key() {
                return Some(vec![column]);
            }
        }

        None
    }
}
