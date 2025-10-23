//! Submodule for the `pg_catalog.pg_statistic_ext` table schema.

diesel::table! {
    /// `pg_catalog.pg_statistic_ext` — table storing extended planner statistics.
    /// Each row represents an extended statistics object.
    /// Uses `oid` as the primary key.
    pg_catalog.pg_statistic_ext (oid) {
        /// Row identifier.
        oid -> Oid,
        /// The table this statistics object is defined on.
        stxrelid -> Oid,
        /// Name of the statistics object.
        stxname -> Text,
        /// The OID of the namespace that contains this statistics object.
        stxnamespace -> Oid,
        /// Owner of the statistics object.
        stxowner -> Oid,
        /// Statistics target for this object.
        stxstattarget -> Nullable<SmallInt>,
        /// Array of attribute numbers for the columns covered by this statistics object.
        stxkeys -> Array<SmallInt>,
        /// Array showing which types of statistics this object collects.
        stxkind -> Array<Text>,
        /// Expression tree (in nodeToString() representation) for statistics object expressions.
        stxexprs -> Nullable<Text>,
    }
}
