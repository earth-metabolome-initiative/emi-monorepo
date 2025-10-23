//! Submodule for the `pg_catalog.pg_opclass` table schema.

diesel::table! {
    /// `pg_catalog.pg_opclass` — system catalog containing operator classes.
    /// Operator classes define how indexes for a particular data type should behave.
    pg_catalog.pg_opclass (oid) {
        /// OID of the operator class.
        oid -> Oid,
        /// OID of the index access method this operator class is for.
        opcmethod -> Oid,
        /// Name of the operator class.
        opcname -> Text,
        /// OID of the namespace containing this operator class.
        opcnamespace -> Oid,
        /// OID of the owner of the operator class.
        opcowner -> Oid,
        /// OID of the operator family containing this operator class.
        opcfamily -> Oid,
        /// OID of the data type that this operator class indexes.
        opcintype -> Oid,
        /// `true` if this operator class is the default for its data type.
        opcdefault -> Bool,
        /// OID of the data type that the index will actually store (0 if same as opcintype).
        opckeytype -> Oid,
    }
}
