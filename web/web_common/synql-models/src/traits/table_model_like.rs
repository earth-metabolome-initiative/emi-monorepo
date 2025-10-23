//! Submodule providing the `TableModel` trait for SynQL table models.

use synql_core::{
    structs::{InternalDataRef, Workspace},
    traits::TableSynLike,
};
use synql_diesel_schema::traits::TableSchema;

use crate::{structs::TableModel, traits::column_model_like::ColumnModelLike};

/// Name of the module containing the model for a table.
pub const MODEL_MODULE_NAME: &str = "model";

/// Trait representing a SynQL table model.
pub trait TableModelLike: TableSchema<ColumnSyn = <Self as TableModelLike>::ColumnModel> {
    /// The column model associated with the table model.
    type ColumnModel: ColumnModelLike<Table = Self, Database = Self::Database>;

    /// Returns the name of the crate which will contain the diesel model for
    /// the table.
    fn table_model_crate_name(&self) -> String {
        format!("{}_model", self.table_singular_snake_name())
    }

    /// Returns the
    /// [`InternalData<'data>`](synql_core::structs::InternalData)
    /// representing the model for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    ///
    /// # Examples
    fn model<'table, 'data>(
        &'table self,
        workspace: &'table Workspace<'data>,
        database: &'table Self::Database,
    ) -> TableModel<'data, 'table, Self>
    where
        Self: 'data,
    {
        TableModel::new(self, workspace, database)
    }

    /// Returns a reference to the model module ref for the table.
    fn model_ref<'data>(&self, workspace: &Workspace<'data>) -> Option<InternalDataRef<'data>> {
        let crate_ref = workspace.internal_crate(&self.table_model_crate_name())?;
        Some(InternalDataRef::new(
            crate_ref,
            crate_ref.internal_data(&self.table_singular_camel_name())?,
        ))
    }
}

impl<T: TableSynLike> TableModelLike for T
where
    T::ColumnSyn: ColumnModelLike<Table = T>,
{
    type ColumnModel = T::ColumnSyn;
}
