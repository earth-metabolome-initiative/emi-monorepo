//! Submodule for the `information_schema.columns` view schema.

diesel::table! {
    /// `information_schema.columns` — view containing one row for each column of accessible
    /// tables or views in the current database. Provides detailed metadata about
    /// column types, defaults, constraints, and identity/auto-generation properties.
    information_schema.columns (table_catalog, table_schema, table_name, column_name) {
        /// Name of the database containing the table.
        table_catalog -> Text,
        /// Name of the schema containing the table.
        table_schema -> Text,
        /// Name of the table containing the column.
        table_name -> Text,
        /// Name of the column.
        column_name -> Text,
        /// Position of the column in the table (1-based).
        ordinal_position -> Integer,
        /// Default expression for the column, if any; otherwise `NULL`.
        column_default -> Nullable<Text>,
        /// "YES" if the column allows NULL values, "NO" otherwise.
        #[sql_name = "is_nullable"]
        __is_nullable -> Text,
        /// Data type of the column (e.g., "integer", "text", "timestamp").
        data_type -> Text,
        /// Maximum length for character columns (number of characters); `NULL` if not applicable.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum length in bytes for character columns; `NULL` if not applicable.
        character_octet_length -> Nullable<Integer>,
        /// Catalog containing the character set of the column; `NULL` if not applicable.
        character_set_catalog -> Nullable<Text>,
        /// Schema containing the character set of the column; `NULL` if not applicable.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set of the column; `NULL` if not applicable.
        character_set_name -> Nullable<Text>,
        /// Catalog containing the collation of the column; `NULL` if not applicable.
        collation_catalog -> Nullable<Text>,
        /// Schema containing the collation of the column; `NULL` if not applicable.
        collation_schema -> Nullable<Text>,
        /// Name of the collation of the column; `NULL` if not applicable.
        collation_name -> Nullable<Text>,
        /// Precision for numeric columns; `NULL` if not applicable.
        numeric_precision -> Nullable<Integer>,
        /// Radix (base) for numeric columns; typically 2 or 10; `NULL` if not applicable.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric columns; `NULL` if not applicable.
        numeric_scale -> Nullable<Integer>,
        /// Precision for date/time columns (number of fractional seconds digits); `NULL` if not applicable.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval columns (e.g., "YEAR TO MONTH"); `NULL` if not applicable.
        interval_type -> Nullable<Text>,
        /// Precision of interval columns; `NULL` if not applicable.
        interval_precision -> Nullable<Integer>,
        /// Catalog of the underlying user-defined type (UDT); `NULL` if not applicable.
        udt_catalog -> Nullable<Text>,
        /// Schema of the UDT; `NULL` if not applicable.
        udt_schema -> Nullable<Text>,
        /// Name of the UDT; `NULL` if not applicable.
        udt_name -> Nullable<Text>,
        /// Catalog of the domain type; `NULL` if not a domain.
        domain_catalog -> Nullable<Text>,
        /// Schema of the domain type; `NULL` if not a domain.
        domain_schema -> Nullable<Text>,
        /// Name of the domain type; `NULL` if not a domain.
        domain_name -> Nullable<Text>,
        /// Catalog of the scope of a reference attribute (foreign key); `NULL` if not applicable.
        scope_catalog -> Nullable<Text>,
        /// Schema of the scope of a reference attribute; `NULL` if not applicable.
        scope_schema -> Nullable<Text>,
        /// Name of the scope of a reference attribute; `NULL` if not applicable.
        scope_name -> Nullable<Text>,
        /// Maximum cardinality for array types; `NULL` if not applicable.
        maximum_cardinality -> Nullable<Integer>,
        /// DTD identifier for the column; `NULL` if not applicable.
        dtd_identifier -> Nullable<Text>,
        /// "YES" if the column is self-referencing (references its own table); `NULL` if not applicable.
        is_self_referencing -> Nullable<Text>,
        /// "YES" if the column is an identity column; "NO" otherwise.
        is_identity -> Nullable<Text>,
        /// "ALWAYS" or "BY DEFAULT" for identity columns; `NULL` otherwise.
        identity_generation -> Nullable<Text>,
        /// Start value of the identity sequence; `NULL` if not an identity column.
        identity_start -> Nullable<Text>,
        /// Increment of the identity sequence; `NULL` if not an identity column.
        identity_increment -> Nullable<Text>,
        /// Maximum value of the identity sequence; `NULL` if not an identity column.
        identity_maximum -> Nullable<Text>,
        /// Minimum value of the identity sequence; `NULL` if not an identity column.
        identity_minimum -> Nullable<Text>,
        /// "YES" if identity sequence cycles, "NO" otherwise; `NULL` if not an identity column.
        identity_cycle -> Nullable<Text>,
        /// "ALWAYS", "BY DEFAULT", or empty string for computed/generated columns.
        is_generated -> Text,
        /// Expression used to generate a computed column; `NULL` if not generated.
        generation_expression -> Nullable<Text>,
        /// "YES" if column is updatable, "NO" otherwise.
        is_updatable -> Text,
    }
}

use super::key_column_usage::key_column_usage;
diesel::allow_tables_to_appear_in_same_query!(columns, key_column_usage);

use super::table_constraints::table_constraints;
diesel::allow_tables_to_appear_in_same_query!(columns, table_constraints);

use super::constraint_column_usage::constraint_column_usage;
diesel::allow_tables_to_appear_in_same_query!(columns, constraint_column_usage);

use super::tables::tables;
diesel::allow_tables_to_appear_in_same_query!(columns, tables);

use crate::schema::pg_catalog::pg_attribute::pg_attribute;
diesel::allow_tables_to_appear_in_same_query!(columns, pg_attribute);

use crate::schema::pg_catalog::pg_type::pg_type;
diesel::allow_tables_to_appear_in_same_query!(columns, pg_type);

use crate::schema::pg_catalog::pg_class::pg_class;
diesel::allow_tables_to_appear_in_same_query!(columns, pg_class);

use crate::schema::pg_catalog::pg_namespace::pg_namespace;
diesel::allow_tables_to_appear_in_same_query!(columns, pg_namespace);

use crate::schema::pg_catalog::pg_index::pg_index;
diesel::allow_tables_to_appear_in_same_query!(columns, pg_index);
