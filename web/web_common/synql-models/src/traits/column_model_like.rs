//! Submodule providing the `ColumnModel` trait for SynQL struct attributes.

use synql_core::{
    prelude::Builder,
    structs::{DataVariantRef, Documentation, InternalAttribute, Workspace},
    traits::ColumnSynLike,
};

/// Trait representing a SynQL column model.
pub trait ColumnModelLike: ColumnSynLike {
    /// Returns the
    /// [`InternalAttribute`](synql_core::structs::InternalAttribute)
    /// representing the attribute for the column.
    ///
    /// # Arguments
    ///
    /// * `database` - The database connection to use to query the column model.
    /// * `workspace` - The workspace where the column is defined.
    fn attribute<'data>(&self, database: &Self::DB, workspace: &Workspace) -> InternalAttribute {
        let mut attribute_type: DataVariantRef = self
            .external_postgres_type(workspace, database)
            .expect("Failed to find external type")
            .into();
        if self.is_nullable(database) {
            attribute_type = attribute_type.optional();
        }

        InternalAttribute::new()
            .name(&self.column_snake_name())
            .expect("Failed to set name")
            .documentation(
                Documentation::new()
                    .documentation(&self.column_doc(database).map(|s| s.to_owned()).unwrap_or_else(
                        || format!("TODO!: document column {}", self.column_name(),),
                    ))
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .public()
            .ty(attribute_type)
            .build()
            .expect("Failed to build column attribute")
    }
}

impl<T: ColumnSynLike> ColumnModelLike for T {}
