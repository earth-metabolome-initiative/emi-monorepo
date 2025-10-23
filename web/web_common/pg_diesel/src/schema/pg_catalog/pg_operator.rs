//! Submodule for the `pg_operator` table schema.

diesel::table! {
    /// `pg_operator` — system catalog containing one row for each operator in the database.
    /// Stores information about built-in and user-defined operators, including their operand
    /// types, result type, associated functions, and optimizer semantics.
    pg_catalog.pg_operator (oid, oprname, oprnamespace) {
        /// OID of the operator.
        oid -> Oid,
        /// Name of the operator (e.g., `+`, `=`, `@>`).
        oprname -> Text,
        /// OID of the namespace (schema) containing the operator.
        oprnamespace -> Oid,
        /// OID of the role that owns the operator.
        oprowner -> Oid,
        /// Operator kind:
        /// 'b' = binary (infix),
        /// 'l' = left unary (prefix),
        /// 'r' = right unary (postfix).
        oprkind -> Char,
        /// `true` if the operator can be used in merge joins.
        oprcanmerge -> Bool,
        /// `true` if the operator can be used in hash joins.
        oprcanhash -> Bool,
        /// OID of the data type of the left operand; 0 if none.
        oprleft -> Oid,
        /// OID of the data type of the right operand; 0 if none.
        oprright -> Oid,
        /// OID of the result data type of the operator.
        oprresult -> Oid,
        /// OID of the operator that is the commutator of this one; 0 if none.
        oprcom -> Oid,
        /// OID of the operator that is the negator of this one; 0 if none.
        oprnegate -> Oid,
        /// OID of the function that implements the operator.
        oprcode -> Oid,
        /// OID of the restriction selectivity estimation function for this operator; 0 if none.
        oprrest -> Oid,
        /// OID of the join selectivity estimation function for this operator; 0 if none.
        oprjoin -> Oid,
    }
}

use super::pg_depend::pg_depend;
diesel::allow_tables_to_appear_in_same_query!(pg_operator, pg_depend);

use super::pg_extension::pg_extension;
diesel::allow_tables_to_appear_in_same_query!(pg_operator, pg_extension);
