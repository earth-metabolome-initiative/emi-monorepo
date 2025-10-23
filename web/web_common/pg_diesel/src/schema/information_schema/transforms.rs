//! `transforms` view from `information_schema`.

diesel::table! {
    /// `information_schema.transforms` — view containing information about
    /// transform functions for user-defined types. Contains one row for each
    /// transform function that transforms between a user-defined type and SQL data types.
    information_schema.transforms (udt_catalog, udt_schema, udt_name, specific_catalog, specific_schema, specific_name, transform_type) {
        /// Name of the database (catalog) containing the user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Name of the schema containing the user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        udt_name -> Nullable<Text>,
        /// Name of the database (catalog) containing the transform function.
        specific_catalog -> Nullable<Text>,
        /// Name of the schema containing the transform function.
        specific_schema -> Nullable<Text>,
        /// Name of the transform function.
        specific_name -> Nullable<Text>,
        /// Group name of the transform (typically for related transforms).
        group_name -> Nullable<Text>,
        /// Type of transform ("FROM SQL" or "TO SQL").
        transform_type -> Nullable<Text>,
    }
}
