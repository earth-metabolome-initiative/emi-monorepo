//! Submodule for the `pg_catalog.pg_proc` table schema.

diesel::table! {
    /// `pg_catalog.pg_proc` — system catalog containing one row per function (procedure) in the database.
    /// Provides metadata about function name, owner, language, arguments, return type, cost, volatility, and other attributes.
    pg_catalog.pg_proc (oid, proname, pronamespace) {
        /// OID of the function.
        oid -> Oid,
        /// Name of the function.
        proname -> Text,
        /// OID of the namespace (schema) containing the function.
        pronamespace -> Oid,
        /// OID of the user who owns the function.
        proowner -> Oid,
        /// OID of the language the function is written in (e.g., SQL, PL/pgSQL, C).
        prolang -> Oid,
        /// Estimated execution cost of the function (used by the planner).
        procost -> Float,
        /// Estimated number of rows returned by the function (used by the planner).
        prorows -> Float,
        /// OID of the variadic argument type, or 0 if none.
        provariadic -> Oid,
        /// OID of the support function (internal use for certain index types); 0 if none.
        prosupport -> Oid,
        /// Function kind: 'f' = normal function, 'p' = procedure, 'a' = aggregate, 'w' = window.
        prokind -> Char,
        /// `true` if the function is SECURITY DEFINER.
        prosecdef -> Bool,
        /// `true` if the function is leakproof (safe for optimization in certain contexts).
        proleakproof -> Bool,
        /// `true` if the function is strict (returns NULL when any argument is NULL).
        proisstrict -> Bool,
        /// `true` if the function returns a set of values.
        proretset -> Bool,
        /// Volatility category of the function: 'i' = immutable, 's' = stable, 'v' = volatile.
        provolatile -> Char,
        /// Parallel safety category: 'u' = unsafe, 's' = safe, 'r' = restricted.
        proparallel -> Char,
        /// Number of arguments the function accepts.
        pronargs -> SmallInt,
        /// Number of arguments with default values.
        pronargdefaults -> SmallInt,
        /// OID of the return type of the function.
        prorettype -> Oid,
        /// Array of argument type OIDs (length = pronargs).
        proargtypes -> Array<Oid>,
        /// Array of all argument type OIDs, including IN/OUT arguments; `NULL` if none.
        proallargtypes -> Nullable<Array<Oid>>,
        /// Array of argument modes: 'i' = IN, 'o' = OUT, 'b' = INOUT, 'v' = VARIADIC; `NULL` if none.
        proargmodes -> Nullable<Array<Char>>,
        /// Array of argument names; `NULL` if none.
        proargnames -> Nullable<Array<Text>>,
        /// Default values for arguments (as a node tree); `NULL` if no defaults.
        proargdefaults -> Nullable<Text>,
        /// Array of type OIDs for output of polymorphic functions; `NULL` if none.
        protrftypes -> Nullable<Array<Oid>>,
        /// Function source code (SQL or language-specific text).
        prosrc -> Text,
        /// Binary representation of the function (for C-language functions); `NULL` if not applicable.
        probin -> Nullable<Bytea>,
        /// SQL body of the function; `NULL` if not applicable.
        prosqlbody -> Nullable<Text>,
        /// Array of configuration parameters for the function; `NULL` if none.
        proconfig -> Nullable<Array<Text>>,
        /// Access privileges for the function.
        proacl -> Nullable<Array<Text>>,
    }
}
