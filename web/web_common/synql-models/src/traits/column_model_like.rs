//! Submodule providing the `ColumnModel` trait for SynQL struct attributes.

use synql_core::{
    prelude::Builder,
    structs::{DataVariantRef, InternalAttribute, Workspace},
    traits::ColumnSynLike,
};

/// Trait representing a SynQL column model.
pub trait ColumnModelLike: ColumnSynLike {
    /// Returns the
    /// [`InternalAttribute<'data>`](synql_core::structs::InternalAttribute)
    /// representing the attribute for the column.
    ///
    /// # Arguments
    ///
    /// * `database` - The database connection to use to query the column model.
    /// * `workspace` - The workspace where the column is defined.
    fn attribute<'data>(
        &self,
        database: &Self::DB,
        workspace: &Workspace<'data>,
    ) -> InternalAttribute<'data> {
        let mut attribute_type: DataVariantRef<'data> =
            self.external_postgres_type(workspace, database).expect("Failed to find external type").into();
        if self.is_nullable(database) {
            attribute_type = attribute_type.optional();
        }

        let mut builder = InternalAttribute::new()
            .name(self.column_snake_name())
            .expect("Failed to set name")
            .public()
            .ty(attribute_type);
        
        if let Some(documentation) = self.column_doc(database) {
            builder = builder.documentation(documentation).expect("Failed to set documentation");
        }
        builder.build().expect("Failed to build column attribute")
    }
}

impl<T: ColumnSynLike> ColumnModelLike for T {}
