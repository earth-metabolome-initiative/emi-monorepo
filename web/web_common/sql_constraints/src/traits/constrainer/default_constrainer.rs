//! Submodule defining a default constrainer with all available constraints.

use sql_traits::traits::DatabaseLike;

use crate::{
    constraints::{
        CompatibleForeignKey, HasPrimaryKey, LowercaseColumnName, LowercaseForeignKeyName,
        LowercaseTableName, NoForbiddenColumnInExtension, NonCompositePrimaryKeyNamedId,
        NonRedundantExtensionDag, PluralTableName, SingularColumnName, SnakeCaseColumnName,
        SnakeCaseTableName, UniqueCheckConstraint, UniqueColumnNamesInExtensionGraph,
        UniqueForeignKey, UniqueUniqueIndex,
    },
    prelude::{
        ExtensionForeignKeyOnDeleteCascade, NoNegationCheckConstraint, NoRustKeywordColumnName,
        NoRustKeywordForeignKeyName, NoRustKeywordTableName, NoTautologicalCheckConstraint,
        PrimaryKeyReferenceEndsWithId, ReferencesUniqueIndex,
    },
    traits::Constrainer,
};

/// A constrainer that comes pre-configured with all available constraints.
///
/// This struct provides a `Default` implementation that registers all available
/// table, column, and foreign key constraints. This is useful for ensuring
/// comprehensive validation of database schemas.
///
/// # Available Constraints
///
/// ## Table Constraints
/// - [`HasPrimaryKey`]: Ensures all tables have a primary key
/// - [`LowercaseTableName`]: Ensures table names are lowercase
/// - [`SnakeCaseTableName`]: Ensures table names follow `snake_case` convention
/// - [`PluralTableName`]: Ensures table names are plural
/// - [`NoForbiddenColumnInExtension`]: Prevents forbidden columns in extended
///   tables
/// - [`NonRedundantExtensionDag`]: Ensures no redundant edges in extension
///   hierarchy
/// - [`UniqueCheckConstraint`]: Ensures check constraint names are unique
/// - [`UniqueColumnNamesInExtensionGraph`]: Ensures column names are unique
///   across extension graphs
/// - [`UniqueForeignKey`]: Ensures foreign key signatures are unique
/// - [`UniqueUniqueIndex`]: Ensures unique index names are unique
///
/// ## Column Constraints
/// - [`LowercaseColumnName`]: Ensures column names are lowercase
/// - [`NonCompositePrimaryKeyNamedId`]: Ensures non-composite primary keys are
///   named "id"
/// - [`SnakeCaseColumnName`]: Ensures column names follow `snake_case`
///   convention
/// - [`SingularColumnName`]: Ensures column names are singular
///
/// ## Foreign Key Constraints
/// - [`CompatibleForeignKey`]: Ensures foreign key columns are type-compatible
/// - [`LowercaseForeignKeyName`]: Ensures foreign key names are lowercase
///
/// # Example
///
/// ```
/// use sql_constraints::prelude::*;
///
/// let constrainer = DefaultConstrainer::<ParserDB>::default();
/// // All constraints are now registered and ready to use
/// ```
pub struct DefaultConstrainer<DB: DatabaseLike> {
    /// The underlying generic constrainer holding all constraints.
    constrainer: super::generic_constrainer::GenericConstrainer<DB>,
}

impl<DB: DatabaseLike + 'static> Default for DefaultConstrainer<DB>
where
    DB::Column: 'static,
{
    fn default() -> Self {
        let mut constrainer = super::generic_constrainer::GenericConstrainer::default();

        // Register all table constraints
        constrainer.register_table_constraint(Box::new(HasPrimaryKey::default()));
        constrainer.register_table_constraint(Box::new(LowercaseTableName::default()));
        constrainer.register_table_constraint(Box::new(SnakeCaseTableName::default()));
        constrainer.register_table_constraint(Box::new(PluralTableName::default()));
        constrainer.register_table_constraint(Box::new(NoRustKeywordTableName::default()));
        constrainer.register_table_constraint(Box::new(NoTautologicalCheckConstraint::default()));
        constrainer.register_table_constraint(Box::new(NoNegationCheckConstraint::default()));
        constrainer.register_table_constraint(Box::new(NoForbiddenColumnInExtension::new(
            "most_concrete_table",
        )));
        constrainer.register_table_constraint(Box::new(NonRedundantExtensionDag::default()));
        constrainer.register_table_constraint(Box::new(UniqueCheckConstraint::default()));
        constrainer
            .register_table_constraint(Box::new(UniqueColumnNamesInExtensionGraph::default()));
        constrainer.register_table_constraint(Box::new(UniqueForeignKey::default()));
        constrainer.register_table_constraint(Box::new(UniqueUniqueIndex::default()));

        // Register all column constraints
        constrainer.register_column_constraint(Box::new(LowercaseColumnName::default()));
        constrainer.register_column_constraint(Box::new(NonCompositePrimaryKeyNamedId::default()));
        constrainer.register_column_constraint(Box::new(SnakeCaseColumnName::default()));
        constrainer.register_column_constraint(Box::new(SingularColumnName::default()));
        constrainer.register_column_constraint(Box::new(NoRustKeywordColumnName::default()));

        // Register all foreign key constraints
        constrainer.register_foreign_key_constraint(Box::new(CompatibleForeignKey::default()));
        constrainer.register_foreign_key_constraint(Box::new(LowercaseForeignKeyName::default()));
        constrainer.register_foreign_key_constraint(Box::new(ReferencesUniqueIndex::default()));
        constrainer
            .register_foreign_key_constraint(Box::new(PrimaryKeyReferenceEndsWithId::default()));
        constrainer.register_foreign_key_constraint(Box::new(
            ExtensionForeignKeyOnDeleteCascade::default(),
        ));
        constrainer
            .register_foreign_key_constraint(Box::new(NoRustKeywordForeignKeyName::default()));

        Self { constrainer }
    }
}

impl<DB: DatabaseLike + 'static> Constrainer for DefaultConstrainer<DB>
where
    DB::Column: 'static,
{
    type Database = DB;

    fn table_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::TableConstraint<Database = Self::Database>> {
        self.constrainer.table_constraints()
    }

    fn column_constraints(
        &self,
    ) -> impl Iterator<
        Item = &dyn crate::traits::ColumnConstraint<
            Column = <Self::Database as DatabaseLike>::Column,
        >,
    > {
        self.constrainer.column_constraints()
    }

    fn foreign_key_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::ForeignKeyConstraint<Database = Self::Database>>
    {
        self.constrainer.foreign_key_constraints()
    }

    fn register_table_constraint(
        &mut self,
        constraint: Box<dyn crate::traits::TableConstraint<Database = Self::Database>>,
    ) {
        self.constrainer.register_table_constraint(constraint);
    }

    fn register_column_constraint(
        &mut self,
        constraint: Box<
            dyn crate::traits::ColumnConstraint<Column = <Self::Database as DatabaseLike>::Column>,
        >,
    ) {
        self.constrainer.register_column_constraint(constraint);
    }

    fn register_foreign_key_constraint(
        &mut self,
        constraint: Box<dyn crate::traits::ForeignKeyConstraint<Database = Self::Database>>,
    ) {
        self.constrainer.register_foreign_key_constraint(constraint);
    }
}
