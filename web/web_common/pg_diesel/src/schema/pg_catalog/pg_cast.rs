//! Submodule for the `pg_catalog.pg_cast` table schema.

diesel::table! {
    /// `pg_catalog.pg_cast` — catalog of data type casts.
    /// Contains information about data type conversion functions.
    pg_catalog.pg_cast (oid) {
        /// OID of this cast (primary key).
        oid -> diesel::sql_types::Oid,
        /// OID of the source data type.
        castsource -> diesel::sql_types::Oid,
        /// OID of the target data type.
        casttarget -> diesel::sql_types::Oid,
        /// OID of the cast function (0 if no function needed).
        castfunc -> diesel::sql_types::Oid,
        /// Context in which cast can be invoked (e=explicit, a=assignment, i=implicit).
        castcontext -> Text,
        /// Method of cast (f=function, i=inout, b=binary coercible).
        castmethod -> Text,
    }
}
