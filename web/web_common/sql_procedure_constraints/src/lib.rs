//! Crate providing constraints to validate tables in the `procedures` and
//! `procedure_templates` DAGs.

use sql_constraints::traits::Constrainer;

pub mod column_constraints;
pub mod table_constraints;

/// Prelude module re-exporting commonly used items from the crate.
pub mod prelude {
    pub use sql_constraints::prelude::*;

    pub use crate::{column_constraints::*, table_constraints::*};
}

/// Function to register all procedure-related constraints to a given
/// constrainer.
pub fn register_procedure_constraints<DB: sql_traits::traits::DatabaseLike + 'static>(
    constrainer: &mut impl Constrainer<Database = DB>,
) {
    constrainer.register_table_constraint(Box::new(
        table_constraints::MatchingProcedureAndTemplateTables::<DB>::default(),
    ));
    constrainer.register_table_constraint(Box::new(
        table_constraints::ProcedureDescendantNaming::<DB>::default(),
    ));
    constrainer.register_table_constraint(Box::new(
        table_constraints::ProcedureTemplateAssetModelUniqueIndex::<DB>::default(),
    ));
    constrainer.register_table_constraint(Box::new(
        table_constraints::ProcedureTemplateDescendantNaming::<DB>::default(),
    ));

    constrainer.register_column_constraint(Box::new(column_constraints::AssetColumnNaming::<
        DB::Column,
    >::default()));
    constrainer.register_column_constraint(Box::new(column_constraints::AssetModelColumnNaming::<
        DB::Column,
    >::default()));
    constrainer.register_column_constraint(Box::new(
        column_constraints::HorizontalAssetModelForeignKey::<DB::Column>::default(),
    ));
    constrainer.register_column_constraint(Box::new(
        column_constraints::MatchingAssetModelColumns::<DB::Column>::default(),
    ));
    constrainer.register_column_constraint(Box::new(
        column_constraints::ProcedureTemplateAssetModelColumnNaming::<DB::Column>::default(),
    ));
}
