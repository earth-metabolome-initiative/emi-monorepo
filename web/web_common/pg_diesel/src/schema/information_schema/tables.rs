//! Submodule for the `information_schema.tables` view schema.

diesel::table! {
    /// `information_schema.tables` — view containing one row for each table or view
    /// in the current database that the current user has access to. Includes
    /// metadata such as schema, type, insertability, and user-defined type info.
    information_schema.tables (table_catalog, table_schema, table_name) {
        /// Name of the database containing the table (always the current database).
        table_catalog -> Text,
        /// Name of the schema that contains the table.
        table_schema -> Text,
        /// Name of the table.
        table_name -> Text,
        /// Table type: "BASE TABLE" (ordinary table), "VIEW", "FOREIGN TABLE", or "LOCAL TEMPORARY".
        table_type -> Text,
        /// For self-referencing tables, the name of the designated "self-referencing" column;
        /// otherwise `NULL`.
        self_referencing_column_name -> Nullable<Text>,
        /// Indicates how values in `self_referencing_column_name` are generated:
        /// "SYSTEM GENERATED", "USER GENERATED", or `NULL`.
        reference_generation -> Nullable<Text>,
        /// Catalog name of the underlying user-defined type if the table is typed; otherwise `NULL`.
        user_defined_type_catalog -> Nullable<Text>,
        /// Schema name of the underlying user-defined type if the table is typed; otherwise `NULL`.
        user_defined_type_schema -> Nullable<Text>,
        /// Name of the underlying user-defined type if the table is typed; otherwise `NULL`.
        user_defined_type_name -> Nullable<Text>,
        /// "YES" if the table is insertable into (e.g., a base table or certain updatable views);
        /// "NO" otherwise.
        is_insertable_into -> Text,
        /// "YES" if the table is a typed table associated with a user-defined type; "NO" otherwise.
        is_typed -> Text,
        /// For updatable views: specifies the action taken on commit ("PRESERVE" or "DELETE");
        /// otherwise `NULL`.
        commit_action -> Nullable<Text>,
    }
}

use crate::schema::pg_catalog::pg_class::pg_class;
diesel::allow_tables_to_appear_in_same_query!(tables, pg_class);

use super::constraint_table_usage::constraint_table_usage;
diesel::allow_tables_to_appear_in_same_query!(tables, constraint_table_usage);

use super::table_constraints::table_constraints;
diesel::allow_tables_to_appear_in_same_query!(tables, table_constraints);
