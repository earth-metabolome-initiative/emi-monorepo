//! Submodule for the `pg_type` table schema.

diesel::table! {
    /// `pg_type` — system catalog containing a row for each data type in the database.
    /// Includes built-in types, user-defined types, array types, composite types, domains, and range types.
    pg_catalog.pg_type (oid, typname, typnamespace) {
        /// OID of the data type.
        oid -> Oid,
        /// Name of the data type.
        typname -> Text,
        /// OID of the namespace (schema) containing the type.
        typnamespace -> Oid,
        /// OID of the owner (role) of the type.
        typowner -> Oid,
        /// Internal length of the type:
        /// -1 = variable-length, -2 = C-string.
        typlen -> SmallInt,
        /// `true` if the type is passed by value, `false` if by reference.
        typbyval -> Bool,
        /// Type kind:
        /// 'b' = base, 'c' = composite, 'd' = domain, 'e' = enum,
        /// 'p' = pseudo, 'r' = range, 'm' = multirange.
        typtype -> Char,
        /// General category of the type (for parser and planner heuristics).
        typcategory -> Char,
        /// `true` if the type is a preferred type within its category.
        typispreferred -> Bool,
        /// `true` if the type is defined, `false` if only a placeholder.
        typisdefined -> Bool,
        /// Delimiter character used for array elements of this type.
        typdelim -> Char,
        /// OID of the `pg_class` entry describing the table for a composite type, or 0 if not composite.
        typrelid -> Oid,
        /// OID of the element type if this is an array type, else 0.
        typelem -> Oid,
        /// OID of the array type having this type as element, or 0 if none.
        typarray -> Oid,
        /// OID of the input function for the type.
        typinput -> Oid,
        /// OID of the output function for the type.
        typoutput -> Oid,
        /// OID of the binary input function for the type.
        typreceive -> Oid,
        /// OID of the binary output function for the type.
        typsend -> Oid,
        /// OID of the typmod input function, or 0 if none.
        typmodin -> Oid,
        /// OID of the typmod output function, or 0 if none.
        typmodout -> Oid,
        /// OID of the type's custom ANALYZE function, or 0 if none.
        typanalyze -> Oid,
        /// OID of the type's subscripting function, or 0 if none.
        typsubscript -> Oid,
        /// Alignment requirement: 'c' = char, 's' = short, 'i' = int, 'd' = double.
        typalign -> Char,
        /// Storage strategy: 'p' = plain, 'e' = external, 'm' = main, 'x' = extended.
        typstorage -> Char,
        /// `true` if the type is declared NOT NULL.
        typnotnull -> Bool,
        /// OID of the base type if this is a domain, else 0.
        typbasetype -> Oid,
        /// Type modifier of the base type if this is a domain, else -1.
        typtypmod -> Integer,
        /// Number of array dimensions for a domain over an array type, else 0.
        typndims -> Integer,
        /// OID of the collation of the type, or 0 if none.
        typcollation -> Oid,
        /// Expression tree for the default value, in nodeToString() representation; `NULL` if none.
        typdefaultbin -> Nullable<Bytea>,
        /// Human-readable text form of the default value; `NULL` if none.
        typdefault -> Nullable<Text>,
        /// Access privileges for the type.
        typacl -> Nullable<Array<Text>>,
    }
}

use super::pg_class::pg_class;
diesel::allow_tables_to_appear_in_same_query!(pg_type, pg_class);

use super::pg_namespace::pg_namespace;
diesel::allow_tables_to_appear_in_same_query!(pg_type, pg_namespace);
