//! Submodule for the `pg_catalog.pg_amop` table schema.

diesel::table! {
    /// `pg_catalog.pg_amop` — catalog of access method operators.
    /// Contains information about operators that can be used with particular access method operator families.
    pg_catalog.pg_amop (oid) {
        /// OID of this entry (primary key).
        oid -> diesel::sql_types::Oid,
        /// OID of operator family this entry belongs to.
        amopfamily -> diesel::sql_types::Oid,
        /// Left-hand input data type of operator.
        amoplefttype -> diesel::sql_types::Oid,
        /// Right-hand input data type of operator.
        amoprighttype -> diesel::sql_types::Oid,
        /// Strategy number associated with operator.
        amopstrategy -> SmallInt,
        /// Purpose of operator (s=search, o=ordering).
        amoppurpose -> Text,
        /// OID of the operator.
        amopopr -> diesel::sql_types::Oid,
        /// OID of access method this operator belongs to.
        amopmethod -> diesel::sql_types::Oid,
        /// OID of sort family, if ordering operator.
        amopsortfamily -> diesel::sql_types::Oid,
    }
}
