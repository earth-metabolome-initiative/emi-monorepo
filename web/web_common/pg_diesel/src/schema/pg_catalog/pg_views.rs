//! Schema for pg_catalog.pg_views view.

diesel::table! {
    use diesel::sql_types::*;

    /// Views
    ///
    /// The view `pg_views` provides access to information about each view in the database.
    pg_catalog.pg_views (schemaname, viewname) {
        /// Name of schema containing the view
        schemaname -> Nullable<Text>,
        /// Name of the view
        viewname -> Nullable<Text>,
        /// Name of the view's owner
        viewowner -> Nullable<Text>,
        /// View definition (SELECT statement)
        definition -> Nullable<Text>,
    }
}
