//! Submodule for the `pg_catalog.pg_aggregate` table schema.

diesel::table! {
    /// `pg_catalog.pg_aggregate` — catalog of aggregate functions.
    /// Contains information about aggregate functions including their transition functions,
    /// final functions, and other properties specific to aggregation.
    pg_catalog.pg_aggregate (aggfnoid) {
        /// OID of the aggregate function (primary key).
        aggfnoid -> diesel::sql_types::Oid,
        /// Kind of aggregate function (normal, ordered-set, hypothetical-set).
        aggkind -> Text,
        /// Number of direct (non-aggregated) arguments.
        aggnumdirectargs -> SmallInt,
        /// Transition function OID.
        aggtransfn -> diesel::sql_types::Oid,
        /// Final function OID.
        aggfinalfn -> diesel::sql_types::Oid,
        /// Combine function OID for parallel aggregation.
        aggcombinefn -> diesel::sql_types::Oid,
        /// Serial function OID for parallel aggregation.
        aggserialfn -> diesel::sql_types::Oid,
        /// Deserial function OID for parallel aggregation.
        aggdeserialfn -> diesel::sql_types::Oid,
        /// Moving-aggregate transition function OID.
        aggmtransfn -> diesel::sql_types::Oid,
        /// Moving-aggregate inverse transition function OID.
        aggminvtransfn -> diesel::sql_types::Oid,
        /// Moving-aggregate final function OID.
        aggmfinalfn -> diesel::sql_types::Oid,
        /// Whether final function wants extra dummy arguments.
        aggfinalextra -> Bool,
        /// Whether moving-aggregate final function wants extra dummy arguments.
        aggmfinalextra -> Bool,
        /// Whether final function modifies transition state.
        aggfinalmodify -> Text,
        /// Whether moving-aggregate final function modifies transition state.
        aggmfinalmodify -> Text,
        /// Associated sort operator OID.
        aggsortop -> diesel::sql_types::Oid,
        /// Data type of transition state.
        aggtranstype -> diesel::sql_types::Oid,
        /// Approximate average size of transition state.
        aggtransspace -> Integer,
        /// Data type of moving-aggregate transition state.
        aggmtranstype -> diesel::sql_types::Oid,
        /// Approximate average size of moving-aggregate transition state.
        aggmtransspace -> Integer,
        /// Initial value of transition state.
        agginitval -> Nullable<Text>,
        /// Initial value of moving-aggregate transition state.
        aggminitval -> Nullable<Text>,
    }
}
