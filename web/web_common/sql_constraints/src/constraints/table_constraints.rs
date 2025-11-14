//! Submodule providing constraint structs that can be applied to tables.

mod lowercase_table_name;
pub use lowercase_table_name::LowercaseTableName;
mod snake_case_table_name;
pub use snake_case_table_name::SnakeCaseTableName;
mod plural_table_name;
pub use plural_table_name::PluralTableName;
mod unique_check_constraint;
pub use unique_check_constraint::UniqueCheckConstraint;
mod unique_unique_index;
pub use unique_unique_index::UniqueUniqueIndex;
mod unique_foreign_key;
pub use unique_foreign_key::UniqueForeignKey;
mod has_primary_key;
pub use has_primary_key::HasPrimaryKey;
mod non_composite_primary_key_named_id;
pub use non_composite_primary_key_named_id::NonCompositePrimaryKeyNamedId;
mod no_forbidden_column_in_extension;
pub use no_forbidden_column_in_extension::NoForbiddenColumnInExtension;
mod non_redundant_extension_dag;
pub use non_redundant_extension_dag::NonRedundantExtensionDag;
mod unique_column_names_in_extension_graph;
pub use unique_column_names_in_extension_graph::UniqueColumnNamesInExtensionGraph;
