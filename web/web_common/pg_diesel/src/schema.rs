//! Schema definitions for PostgreSQL system catalogs and information schema
//! views.
use diesel::prelude::{allow_tables_to_appear_in_same_query, table};

table! {
    /// `pg_catalog.pg_settings` — view exposing PostgreSQL run-time configuration parameters (GUCs).
    /// Each row represents one server configuration parameter and metadata about it:
    /// name, current value, type, source, allowable range, descriptions, and file/location info.
    pg_catalog.pg_settings (name) {
        /// Name of the configuration parameter.
        name -> Text,
        /// Current value of the parameter, formatted as text (units stripped; booleans as "on"/"off").
        setting -> Text,
        /// Measurement unit for the parameter (e.g., "kB", "ms", "s"); `NULL` if unitless.
        unit -> Nullable<Text>,
        /// Logical category used for grouping/documentation (e.g., "Memory", "Logging").
        category -> Text,
        /// Short description summarizing what the parameter controls.
        short_desc -> Text,
        /// Additional explanatory/descriptive text about the parameter; may be `NULL`.
        extra_desc -> Nullable<Text>,
        /// Change context describing when/how the parameter can be set or takes effect
        /// (examples: "internal", "postmaster", "sighup", "backend", "superuser", "user").
        context -> Text,
        /// Parameter data type: "bool", "integer", "real", "enum", or "string".
        vartype -> Text,
        /// Source of the current value (e.g., "default", "configuration file", "command line",
        /// "environment", "session", "database", "user", "override").
        source -> Text,
        /// Minimum allowed value for the parameter (as text); `NULL` if not applicable.
        min_val -> Nullable<Text>,
        /// Maximum allowed value for the parameter (as text); `NULL` if not applicable.
        max_val -> Nullable<Text>,
        /// For enum-typed parameters: array of allowed textual values; `NULL` for non-enums.
        enumvals -> Nullable<Array<Text>>,
        /// Built-in (compiled-in) default value for the parameter; may be `NULL`.
        boot_val -> Nullable<Text>,
        /// The value the parameter would have after a `RESET` (session/system default);
        /// may be `NULL`.
        reset_val -> Nullable<Text>,
        /// File path from which the current setting was read (typically `postgresql.conf`);
        /// `NULL` if not applicable.
        sourcefile -> Nullable<Text>,
        /// Line number within `sourcefile` where this setting appears; `NULL` if unknown.
        sourceline -> Nullable<Integer>,
        /// `true` if the parameter change is pending and requires a server restart to take effect;
        /// `NULL` if not applicable.
        pending_restart -> Nullable<Bool>,
    }
}

table! {
    /// `information_schema.tables` — view containing one row for each table or view
    /// in the current database that the current user has access to. Includes
    /// metadata such as schema, type, insertability, and user-defined type info.
    information_schema.tables (table_catalog, table_schema, table_name) {
        /// Name of the database containing the table (always the current database).
        table_catalog -> Text,
        /// Name of the schema that contains the table.
        table_schema -> Text,
        /// Name of the table.
        table_name -> Text,
        /// Table type: "BASE TABLE" (ordinary table), "VIEW", "FOREIGN TABLE", or "LOCAL TEMPORARY".
        table_type -> Text,
        /// For self-referencing tables, the name of the designated "self-referencing" column;
        /// otherwise `NULL`.
        self_referencing_column_name -> Nullable<Text>,
        /// Indicates how values in `self_referencing_column_name` are generated:
        /// "SYSTEM GENERATED", "USER GENERATED", or `NULL`.
        reference_generation -> Nullable<Text>,
        /// Catalog name of the underlying user-defined type if the table is typed; otherwise `NULL`.
        user_defined_type_catalog -> Nullable<Text>,
        /// Schema name of the underlying user-defined type if the table is typed; otherwise `NULL`.
        user_defined_type_schema -> Nullable<Text>,
        /// Name of the underlying user-defined type if the table is typed; otherwise `NULL`.
        user_defined_type_name -> Nullable<Text>,
        /// "YES" if the table is insertable into (e.g., a base table or certain updatable views);
        /// "NO" otherwise.
        is_insertable_into -> Text,
        /// "YES" if the table is a typed table associated with a user-defined type; "NO" otherwise.
        is_typed -> Text,
        /// For updatable views: specifies the action taken on commit ("PRESERVE" or "DELETE");
        /// otherwise `NULL`.
        commit_action -> Nullable<Text>,
    }
}

table! {
    /// `information_schema.columns` — view containing one row for each column of accessible
    /// tables or views in the current database. Provides detailed metadata about
    /// column types, defaults, constraints, and identity/auto-generation properties.
    information_schema.columns (table_catalog, table_schema, table_name, column_name) {
        /// Name of the database containing the table.
        table_catalog -> Text,
        /// Name of the schema containing the table.
        table_schema -> Text,
        /// Name of the table containing the column.
        table_name -> Text,
        /// Name of the column.
        column_name -> Text,
        /// Position of the column in the table (1-based).
        ordinal_position -> Integer,
        /// Default expression for the column, if any; otherwise `NULL`.
        column_default -> Nullable<Text>,
        /// "YES" if the column allows NULL values, "NO" otherwise.
        #[sql_name = "is_nullable"]
        __is_nullable -> Text,
        /// Data type of the column (e.g., "integer", "text", "timestamp").
        data_type -> Text,
        /// Maximum length for character columns (number of characters); `NULL` if not applicable.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum length in bytes for character columns; `NULL` if not applicable.
        character_octet_length -> Nullable<Integer>,
        /// Precision for numeric columns; `NULL` if not applicable.
        numeric_precision -> Nullable<Integer>,
        /// Radix (base) for numeric columns; typically 2 or 10; `NULL` if not applicable.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric columns; `NULL` if not applicable.
        numeric_scale -> Nullable<Integer>,
        /// Precision for date/time columns (number of fractional seconds digits); `NULL` if not applicable.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval columns (e.g., "YEAR TO MONTH"); `NULL` if not applicable.
        interval_type -> Nullable<Text>,
        /// Precision of interval columns; `NULL` if not applicable.
        interval_precision -> Nullable<Integer>,
        /// Catalog of the underlying user-defined type (UDT); `NULL` if not applicable.
        udt_catalog -> Nullable<Text>,
        /// Schema of the UDT; `NULL` if not applicable.
        udt_schema -> Nullable<Text>,
        /// Name of the UDT; `NULL` if not applicable.
        udt_name -> Nullable<Text>,
        /// Catalog of the scope of a reference attribute (foreign key); `NULL` if not applicable.
        scope_catalog -> Nullable<Text>,
        /// Schema of the scope of a reference attribute; `NULL` if not applicable.
        scope_schema -> Nullable<Text>,
        /// Name of the scope of a reference attribute; `NULL` if not applicable.
        scope_name -> Nullable<Text>,
        /// Maximum cardinality for array types; `NULL` if not applicable.
        maximum_cardinality -> Nullable<Integer>,
        /// DTD identifier for the column; `NULL` if not applicable.
        dtd_identifier -> Nullable<Text>,
        /// "YES" if the column is self-referencing (references its own table); `NULL` if not applicable.
        is_self_referencing -> Nullable<Text>,
        /// "YES" if the column is an identity column; "NO" otherwise.
        is_identity -> Nullable<Text>,
        /// "ALWAYS", "BY DEFAULT", or empty string for computed/generated columns.
        is_generated -> Text,
        /// Expression used to generate a computed column; `NULL` if not generated.
        generation_expression -> Nullable<Text>,
        /// "YES" if column is updatable, "NO" otherwise.
        is_updatable -> Text,
    }
}

table! {
    /// `information_schema.key_column_usage` — view containing one row per column
    /// that is constrained by a primary key, unique key, or foreign key.
    /// Provides metadata linking columns to their constraints and, for foreign keys,
    /// the referenced unique/primary key column.
    information_schema.key_column_usage (table_catalog, table_schema, table_name, column_name) {
        /// Catalog (database) containing the constraint.
        constraint_catalog -> Text,
        /// Schema containing the constraint.
        constraint_schema -> Text,
        /// Name of the constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the table with the constrained column.
        table_catalog -> Text,
        /// Schema containing the table with the constrained column.
        table_schema -> Text,
        /// Name of the table containing the constrained column.
        table_name -> Text,
        /// Name of the constrained column.
        column_name -> Text,
        /// Position of the column within the constraint (1-based).
        ordinal_position -> Integer,
        /// For foreign key constraints: position of this column within the referenced unique or primary key;
        /// `NULL` for primary key or unique constraints.
        position_in_unique_constraint -> Nullable<Integer>,
    }
}

table! {
    /// `information_schema.table_constraints` — view containing one row per table constraint.
    /// Includes primary keys, foreign keys, unique constraints, and check constraints.
    /// Provides metadata such as type, deferrability, enforcement, and null handling.
    information_schema.table_constraints (table_catalog, table_schema, table_name, constraint_name) {
        /// Catalog (database) containing the constraint.
        constraint_catalog -> Text,
        /// Schema containing the constraint.
        constraint_schema -> Text,
        /// Name of the constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the table with the constraint.
        table_catalog -> Text,
        /// Schema containing the table with the constraint.
        table_schema -> Text,
        /// Name of the table containing the constraint.
        table_name -> Text,
        /// Type of the constraint: "PRIMARY KEY", "FOREIGN KEY", "UNIQUE", "CHECK".
        constraint_type -> Text,
        /// "YES" if the constraint is deferrable; "NO" otherwise.
        is_deferrable -> Text,
        /// "YES" if the constraint is initially deferred; "NO" otherwise.
        initially_deferred -> Text,
        /// "YES" if the constraint is enforced, "NO" if it is not.
        enforced -> Text,
        /// "YES" if NULL values are treated as distinct for unique constraints; `NULL` otherwise.
        nulls_distinct -> Nullable<Text>,
    }
}

table! {
    /// `information_schema.referential_constraints` — view containing one row per foreign key constraint.
    /// Provides metadata linking the foreign key to its referenced unique or primary key,
    /// along with referential actions and match options.
    information_schema.referential_constraints (constraint_catalog, constraint_schema, constraint_name) {
        /// Catalog (database) containing the foreign key constraint.
        constraint_catalog -> Text,
        /// Schema containing the foreign key constraint.
        constraint_schema -> Text,
        /// Name of the foreign key constraint.
        constraint_name -> Text,
        /// Catalog of the referenced unique or primary key constraint; `NULL` if not applicable.
        unique_constraint_catalog -> Nullable<Text>,
        /// Schema of the referenced unique or primary key constraint; `NULL` if not applicable.
        unique_constraint_schema -> Nullable<Text>,
        /// Name of the referenced unique or primary key constraint; `NULL` if not applicable.
        unique_constraint_name -> Nullable<Text>,
        /// Match option for the foreign key: "FULL", "PARTIAL", or "SIMPLE".
        match_option -> Text,
        /// Action to perform on update of the referenced key: "NO ACTION", "CASCADE", "SET NULL", "SET DEFAULT", or "RESTRICT".
        update_rule -> Text,
        /// Action to perform on delete of the referenced key: "NO ACTION", "CASCADE", "SET NULL", "SET DEFAULT", or "RESTRICT".
        delete_rule -> Text,
    }
}

table! {
    /// `information_schema.constraint_column_usage` — view containing one row per column
    /// that is used by a table constraint (primary key, unique, foreign key, or check).
    /// Links each constrained column to its constraint metadata.
    information_schema.constraint_column_usage (table_catalog, table_schema, table_name, column_name) {
        /// Catalog (database) containing the constraint.
        constraint_catalog -> Text,
        /// Schema containing the constraint.
        constraint_schema -> Text,
        /// Name of the constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the table with the constrained column.
        table_catalog -> Text,
        /// Schema containing the table with the constrained column.
        table_schema -> Text,
        /// Name of the table containing the constrained column.
        table_name -> Text,
        /// Name of the constrained column.
        column_name -> Text,
    }
}

table! {
    /// `information_schema.constraint_table_usage` — view containing one row per table
    /// that is referenced by a table constraint (primary key, unique, foreign key, or check).
    /// Provides a high-level mapping from constraints to the tables they involve.
    information_schema.constraint_table_usage (table_catalog, table_schema, table_name, constraint_name) {
        /// Catalog (database) containing the constraint.
        constraint_catalog -> Text,
        /// Schema containing the constraint.
        constraint_schema -> Text,
        /// Name of the constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the table referenced by the constraint.
        table_catalog -> Text,
        /// Schema containing the table referenced by the constraint.
        table_schema -> Text,
        /// Name of the table referenced by the constraint.
        table_name -> Text,
    }
}

table! {
    /// `information_schema.check_constraints` — view containing one row per check constraint.
    /// Provides metadata about check constraints defined on tables, including the expression that must hold.
    information_schema.check_constraints (constraint_catalog, constraint_schema, constraint_name) {
        /// Catalog (database) containing the check constraint.
        constraint_catalog -> Text,
        /// Schema containing the check constraint.
        constraint_schema -> Text,
        /// Name of the check constraint.
        constraint_name -> Text,
        /// The Boolean expression that defines the check constraint.
        check_clause -> Text,
    }
}

table! {
    /// `information_schema.domain_constraints` — view containing one row per constraint
    /// defined on a domain. Provides metadata about domain-level constraints,
    /// including deferrability and initial enforcement timing.
    information_schema.domain_constraints (constraint_catalog, constraint_schema, constraint_name) {
        /// Catalog (database) containing the domain constraint.
        constraint_catalog -> Text,
        /// Schema containing the domain constraint.
        constraint_schema -> Text,
        /// Name of the domain constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the domain; `NULL` if not applicable.
        domain_catalog -> Nullable<Text>,
        /// Schema containing the domain; `NULL` if not applicable.
        domain_schema -> Nullable<Text>,
        /// Name of the domain; `NULL` if not applicable.
        domain_name -> Nullable<Text>,
        /// "YES" if the constraint is deferrable; "NO" otherwise.
        is_deferrable -> Text,
        /// "YES" if the constraint is initially deferred; "NO" otherwise.
        initially_deferred -> Text,
    }
}

table! {
    /// `pg_indexes` — view containing one row per index in the database.
    /// Provides metadata about indexes, including their schema, table, and definition.
    pg_indexes (schemaname, tablename, indexname) {
        /// Name of the schema containing the table with the index.
        schemaname -> Text,
        /// Name of the table the index is defined on.
        tablename -> Text,
        /// Name of the index.
        indexname -> Text,
        /// Name of the tablespace the index resides in; `NULL` if using default tablespace.
        tablespace -> Nullable<Text>,
        /// SQL definition of the index (e.g., "CREATE INDEX ...").
        indexdef -> Text,
    }
}

table! {
    /// `pg_index` — system catalog containing one row per index in the database.
    /// Provides low-level metadata about index properties, columns, uniqueness,
    /// and internal system behavior.
    pg_index (indexrelid) {
        /// OID of the index itself.
        indexrelid -> Oid,
        /// OID of the table the index is defined on.
        indrelid -> Oid,
        /// Total number of columns in the index.
        indnatts -> SmallInt,
        /// Number of key columns used for uniqueness checks (excluding included columns in some versions).
        indnkeyatts -> SmallInt,
        /// `true` if the index enforces uniqueness.
        indisunique -> Bool,
        /// `true` if the index is a primary key.
        indisprimary -> Bool,
        /// `true` if the index is an exclusion constraint index.
        indisexclusion -> Bool,
        /// `true` if the index is immediate (not deferred).
        indimmediate -> Bool,
        /// `true` if the index is clustered on the table.
        indisclustered -> Bool,
        /// `true` if the index is valid (can be used by queries).
        indisvalid -> Bool,
        /// `true` if the index is protected against transaction wraparound issues.
        indcheckxmin -> Bool,
        /// `true` if the index is ready for inserts/updates.
        indisready -> Bool,
        /// `true` if the index is live and actively maintained by the system.
        indislive -> Bool,
        /// `true` if the index is the replication identity of the table.
        indisreplident -> Bool,
        /// Array of attribute numbers of table columns included in the index.
        indkey -> Array<SmallInt>,
        /// Array of OIDs for column collations for the index columns.
        indcollation -> Array<Oid>,
        /// Array of operator class OIDs for the index columns.
        indclass -> Array<Oid>,
        /// Array of index options (per-column flags such as DESC, NULLS FIRST, etc.).
        indoption -> Array<SmallInt>,
    }
}

table! {
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
        /// Array of argument default values as text; `NULL` if none.
        proargdefaults -> Nullable<Array<Text>>,
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
    }
}

table! {
    /// `pg_catalog.pg_namespace` — system catalog containing one row per schema (namespace) in the database.
    /// Provides metadata about schema names and their owners.
    pg_catalog.pg_namespace (oid) {
        /// OID of the schema.
        oid -> Oid,
        /// Name of the schema.
        nspname -> Text,
        /// OID of the user who owns the schema.
        nspowner -> Oid,
    }
}

table! {
    /// `pg_catalog.pg_extension` — system catalog containing one row per extension installed in the database.
    /// Provides metadata about extension name, owner, schema, version, and any configuration tables.
    pg_catalog.pg_extension (oid) {
        /// OID of the extension.
        oid -> Oid,
        /// Name of the extension.
        extname -> Text,
        /// OID of the user who owns the extension.
        extowner -> Oid,
        /// OID of the schema that contains objects for the extension.
        extnamespace -> Oid,
        /// `true` if the extension is relocatable (can be moved to a different schema); `false` otherwise.
        extrelocatable -> Bool,
        /// Version of the extension.
        extversion -> Text,
        /// Array of OIDs of configuration tables associated with the extension; `NULL` if none.
        extconfig -> Nullable<Array<Oid>>,
        /// Array of conditions for each configuration table; `NULL` if none.
        extcondition -> Nullable<Array<Text>>,
    }
}

table! {
    /// `pg_catalog.pg_depend` — system catalog containing one row per dependency between database objects.
    /// Tracks how objects depend on each other, e.g., table columns depending on types, or views depending on tables.
    pg_catalog.pg_depend (classid, objid, objsubid) {
        /// OID of the system catalog containing the dependent object (e.g., `pg_class`, `pg_proc`).
        classid -> Oid,
        /// OID of the dependent object.
        objid -> Oid,
        /// Sub-object ID within the object (e.g., column number for table columns), 0 if not applicable.
        objsubid -> Integer,
        /// OID of the system catalog containing the referenced object.
        refclassid -> Oid,
        /// OID of the referenced object.
        refobjid -> Oid,
        /// Sub-object ID within the referenced object, 0 if not applicable.
        refobjsubid -> Integer,
        /// Dependency type:
        /// 'n' = normal, 'a' = automatic, 'i' = internal, 'e' = extension, 'p' = pinned.
        deptype -> Char,
    }
}

table! {
    /// `pg_type` — system catalog containing a row for each data type in the database.
    /// Includes built-in types, user-defined types, array types, composite types, domains, and range types.
    pg_type (oid, typname, typnamespace) {
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
        /// OID of the type’s custom ANALYZE function, or 0 if none.
        typanalyze -> Oid,
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
    }
}

table! {
    /// `pg_operator` — system catalog containing one row for each operator in the database.
    /// Stores information about built-in and user-defined operators, including their operand
    /// types, result type, associated functions, and optimizer semantics.
    pg_operator (oid, oprname, oprnamespace) {
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

table! {
    /// `pg_constraint` — system catalog containing one row for each table, domain, or index constraint.
    /// Stores metadata about primary keys, foreign keys, unique constraints, check constraints,
    /// exclusion constraints, and domain constraints.
    pg_constraint (oid, conname, connamespace) {
        /// OID of the constraint.
        oid -> Oid,
        /// Name of the constraint (unique within the schema).
        conname -> Text,
        /// OID of the namespace (schema) containing the constraint.
        connamespace -> Oid,
        /// Constraint type:
        /// 'c' = check constraint,
        /// 'f' = foreign key,
        /// 'p' = primary key,
        /// 'u' = unique constraint,
        /// 't' = constraint trigger,
        /// 'x' = exclusion constraint.
        contype -> Char,
        /// `true` if the constraint is deferrable.
        condeferrable -> Bool,
        /// `true` if the constraint is deferred by default.
        condeferred -> Bool,
        /// `true` if the constraint has been validated.
        convalidated -> Bool,
        /// OID of the table this constraint belongs to, or 0 if not a table constraint.
        conrelid -> Oid,
        /// OID of the domain this constraint belongs to, or 0 if not a domain constraint.
        contypid -> Oid,
        /// OID of the index supporting this constraint (for unique, primary key, or exclusion constraints), or 0 if none.
        conindid -> Oid,
        /// OID of the parent constraint if this is a child in an inheritance hierarchy, else 0.
        conparentid -> Oid,
        /// OID of the referenced table for foreign key constraints, else 0.
        confrelid -> Oid,
        /// Action on update for foreign key:
        /// 'a' = no action, 'r' = restrict, 'c' = cascade, 'n' = set null, 'd' = set default.
        confupdtype -> Char,
        /// Action on delete for foreign key:
        /// 'a' = no action, 'r' = restrict, 'c' = cascade, 'n' = set null, 'd' = set default.
        confdeltype -> Char,
        /// Match type for foreign key:
        /// 'f' = full, 'p' = partial, 's' = simple.
        confmatchtype -> Char,
        /// `true` if the constraint is defined locally (not inherited).
        conislocal -> Bool,
        /// Number of times this constraint is inherited by child tables.
        coninhcount -> SmallInt,
        /// `true` if the constraint cannot be inherited.
        connoinherit -> Bool,
        /// Column numbers of the constrained columns in the table, or `NULL` if not applicable.
        conkey -> Nullable<Array<SmallInt>>,
        /// Column numbers of the referenced columns in the foreign key’s referenced table, or `NULL`.
        confkey -> Nullable<Array<SmallInt>>,
        /// Equality operators for the pairs of constrained columns (foreign key), or `NULL`.
        conpfeqop -> Nullable<Array<Oid>>,
        /// Equality operators for pairs of parent and child columns in table inheritance, or `NULL`.
        conppeqop -> Nullable<Array<Oid>>,
        /// Equality operators for referenced columns in the foreign key’s referenced table, or `NULL`.
        conffeqop -> Nullable<Array<Oid>>,
        /// Column numbers in the referencing table that are set in `ON DELETE SET NULL` or `SET DEFAULT`, or `NULL`.
        confdelsetcols -> Nullable<Array<SmallInt>>,
        /// Operator OIDs for exclusion constraints; `NULL` if not applicable.
        conexclop -> Nullable<Array<Oid>>,
    }
}

table! {
    /// `pg_class` — system catalog containing one row for each table, index, sequence, view,
    /// materialized view, composite type, and TOAST table.
    /// Stores metadata such as storage, persistence, row count estimates, and relation type.
    pg_class (oid, relname, relnamespace) {
        /// OID of the relation.
        oid -> Oid,
        /// Name of the relation (table, index, etc.).
        relname -> Text,
        /// OID of the namespace (schema) containing this relation.
        relnamespace -> Oid,
        /// OID of the composite type that represents the row type for this relation,
        /// or 0 if none.
        reltype -> Oid,
        /// OID of the underlying type if this is a typed table, else 0.
        reloftype -> Oid,
        /// OID of the role that owns the relation.
        relowner -> Oid,
        /// OID of the access method used (e.g., heap, B-tree, hash).
        relam -> Oid,
        /// Identifier of the relation’s physical storage file (or 0 if none, such as for views).
        relfilenode -> Oid,
        /// OID of the tablespace containing the relation, or 0 for the database’s default tablespace.
        reltablespace -> Oid,
        /// Number of disk pages the relation uses (as of last `ANALYZE`).
        relpages -> Integer,
        /// Estimated number of live rows in the relation (as of last `ANALYZE`).
        reltuples -> Float,
        /// Number of all-visible pages (from visibility map).
        relallvisible -> Integer,
        /// OID of the TOAST table associated with this relation, or 0 if none.
        reltoastrelid -> Oid,
        /// `true` if the relation has (or once had) an index.
        relhasindex -> Bool,
        /// `true` if the relation is shared across all databases (e.g., `pg_database`).
        relisshared -> Bool,
        /// Relation persistence:
        /// 'p' = permanent,
        /// 'u' = unlogged,
        /// 't' = temporary.
        relpersistence -> Char,
        /// Relation kind:
        /// 'r' = ordinary table,
        /// 'i' = index,
        /// 'S' = sequence,
        /// 't' = TOAST table,
        /// 'v' = view,
        /// 'm' = materialized view,
        /// 'c' = composite type,
        /// 'f' = foreign table,
        /// 'p' = partitioned table,
        /// 'I' = partitioned index.
        relkind -> Char,
        /// Number of user columns (attributes) in the relation.
        relnatts -> SmallInt,
        /// Number of check constraints on the relation.
        relchecks -> SmallInt,
        /// `true` if the relation has rules.
        relhasrules -> Bool,
        /// `true` if the relation has triggers.
        relhastriggers -> Bool,
        /// `true` if the relation has child tables (is a parent in inheritance).
        relhassubclass -> Bool,
        /// `true` if row-level security (RLS) is enabled.
        relrowsecurity -> Bool,
        /// `true` if row-level security (RLS) is forced for all users.
        relforcerowsecurity -> Bool,
        /// `true` if the table is currently populated (for materialized views).
        relispopulated -> Bool,
        /// Replica identity setting:
        /// 'd' = default (primary key),
        /// 'n' = nothing,
        /// 'f' = all columns,
        /// 'i' = index.
        relreplident -> Char,
        /// `true` if this relation is a partition of another table.
        relispartition -> Bool,
        /// OID of the relation this one is being rewritten into (during operations like `ALTER TABLE`), or 0.
        relrewrite -> Oid,
    }
}

table! {
    /// `pg_attribute` — system catalog containing one row for each column (attribute) in a table,
    /// index, view, composite type, or foreign table.
    /// Stores metadata about column names, types, storage, nullability, defaults, identity, and inheritance.
    pg_attribute (attrelid, attname, atttypid) {
        /// OID of the relation containing this column (references `pg_class.oid`).
        attrelid -> Oid,
        /// Name of the column.
        attname -> Text,
        /// OID of the column’s data type (references `pg_type.oid`).
        atttypid -> Oid,
        /// Internal storage length of the column type.
        /// -1 = variable-length, -2 = C-string.
        attlen -> SmallInt,
        /// Column number:
        /// Positive = user-defined columns,
        /// 0 = table OID column,
        /// Negative = system columns.
        attnum -> SmallInt,
        /// Column offset in the tuple (for internal use by the executor).
        /// -1 if not cached.
        attcacheoff -> Integer,
        /// Type-specific modifier (atttypmod), often indicating precision/scale for numeric types.
        /// -1 if not applicable.
        atttypmod -> Integer,
        /// Number of array dimensions, if the column is an array; 0 if not.
        attndims -> SmallInt,
        /// `true` if the column’s type is passed by value.
        attbyval -> Bool,
        /// Alignment requirement of the column type:
        /// 'c' = char, 's' = short, 'i' = int, 'd' = double.
        attalign -> Char,
        /// Storage strategy:
        /// 'p' = plain,
        /// 'e' = external,
        /// 'm' = main,
        /// 'x' = extended.
        attstorage -> Char,
        /// `true` if the column is marked NOT NULL.
        attnotnull -> Bool,
        /// `true` if the column has a default value.
        atthasdef -> Bool,
        /// `true` if the column has a missing value defined (for dropped/added columns).
        atthasmissing -> Bool,
        /// Identity column property:
        /// '' = not an identity column,
        /// 'a' = generated always,
        /// 'd' = generated by default.
        attidentity -> Char,
        /// Generated column property:
        /// '' = not generated,
        /// 's' = stored (generated always stored).
        attgenerated -> Char,
        /// `true` if the column has been dropped (still present for compatibility).
        attisdropped -> Bool,
        /// `true` if the column is locally defined (not inherited).
        attislocal -> Bool,
        /// Number of times this column is inherited from parent tables.
        attinhcount -> SmallInt,
        /// OID of the collation of the column, or 0 if not collatable.
        attcollation -> Oid,
        /// Target number of statistics samples for `ANALYZE`;
        /// -1 uses default setting; `NULL` if not set.
        attstattarget -> Nullable<SmallInt>,
        /// Column-level access privileges (ACL); `NULL` if none.
        attacl -> Nullable<Array<Oid>>,
        /// Column-level options (e.g., for storage parameters); `NULL` if none.
        attoptions -> Nullable<Array<Text>>,
        /// Options for foreign data wrapper columns; `NULL` if none.
        attfdwoptions -> Nullable<Array<Text>>,
    }
}

table! {
    /// `pg_trigger` — system catalog containing one row for each trigger in the database.
    /// Stores metadata about trigger definitions, including their name, function, firing conditions,
    /// and relation associations.
    pg_trigger (oid) {
        /// OID of the trigger.
        oid -> Oid,
        /// OID of the relation on which the trigger is defined (references `pg_class.oid`).
        tgrelid -> Oid,
        /// OID of the parent trigger (for inherited triggers), or 0 if none.
        tgparentid -> Oid,
        /// Name of the trigger.
        tgname -> Text,
        /// OID of the function to be executed when the trigger fires (references `pg_proc.oid`).
        tgfoid -> Oid,
        /// Bit mask identifying trigger type and firing conditions:
        /// event type (INSERT/UPDATE/DELETE/TRUNCATE), timing (BEFORE/AFTER/INSTEAD OF),
        /// and row vs. statement level.
        tgtype -> SmallInt,
        /// Enabled state of the trigger:
        /// 'O' = enabled,
        /// 'D' = disabled,
        /// 'R' = enabled for replica,
        /// 'A' = enabled always.
        tgenabled -> Char,
        /// `true` if the trigger is internal (created by the system, not user-defined).
        tgisinternal -> Bool,
        /// OID of the relation referenced by a foreign key constraint trigger, or 0 if not applicable.
        tgconstrrelid -> Oid,
        /// OID of the index referenced by a foreign key constraint trigger, or 0 if not applicable.
        tgconstrindid -> Oid,
        /// OID of the associated constraint (in `pg_constraint`), or 0 if not a constraint trigger.
        tgconstraint -> Oid,
        /// `true` if the trigger is deferrable (can be postponed until transaction commit).
        tgdeferrable -> Bool,
        /// `true` if the trigger is initially deferred by default.
        tginitdeferred -> Bool,
        /// Number of arguments supplied to the trigger function.
        tgnargs -> SmallInt,
        /// Column numbers that this trigger is fired on (for column-specific UPDATE triggers).
        tgattr -> Array<SmallInt>,
        /// Arguments to the trigger function, stored as a byte array.
        tgargs -> Bytea,
        /// Name of the transition table for OLD rows, if defined; `NULL` if none.
        tgoldtable -> Nullable<Text>,
        /// Name of the transition table for NEW rows, if defined; `NULL` if none.
        tgnewtable -> Nullable<Text>,
    }
}

table! {
    /// `pg_enum` — system catalog containing one row for each label of an enum type.
    /// Stores the textual labels and sort ordering of enum members associated with a user-defined enum type.
    pg_enum (oid) {
        /// OID of the enum value.
        oid -> Oid,
        /// OID of the enum type that this label belongs to (references `pg_type.oid`).
        enumtypid -> Oid,
        /// Sort order of this enum label within the enum type.
        /// Lower values sort before higher values.
        /// Used internally for ordering comparisons.
        enumsortorder -> Float,
        /// The textual label of the enum value.
        enumlabel -> Text,
    }
}

table! {
    /// `geometry_columns` — a PostGIS metadata view that contains one row for each geometry column
    /// in the database. Provides information about the table, column name, coordinate dimension,
    /// spatial reference system (SRID), and geometry type.
    geometry_columns (f_table_catalog, f_table_schema, f_table_name, f_geometry_column) {
        /// Name of the database (catalog) that contains the table with the geometry column.
        f_table_catalog -> Text,
        /// Name of the schema that contains the table with the geometry column.
        f_table_schema -> Text,
        /// Name of the table that contains the geometry column.
        f_table_name -> Text,
        /// Name of the column that stores geometry values.
        f_geometry_column -> Text,
        /// Coordinate dimension of the geometry column (e.g., 2 for XY, 3 for XYZ).
        coord_dimension -> Integer,
        /// Spatial reference system identifier (SRID) of the geometry column.
        srid -> Integer,
        /// Type of geometry stored in the column (e.g., `POINT`, `LINESTRING`, `POLYGON`).
        r#type -> Text,
    }
}

table! {
    /// `geography_columns` — a PostGIS metadata view that contains one row for each geography column
    /// in the database. Provides information about the table, column name, coordinate dimension,
    /// spatial reference system (SRID), and geography type.
    geography_columns (f_table_catalog, f_table_schema, f_table_name, f_geography_column) {
        /// Name of the database (catalog) that contains the table with the geography column.
        f_table_catalog -> Text,
        /// Name of the schema that contains the table with the geography column.
        f_table_schema -> Text,
        /// Name of the table that contains the geography column.
        f_table_name -> Text,
        /// Name of the column that stores geography values.
        f_geography_column -> Text,
        /// Coordinate dimension of the geography column (e.g., 2 for XY, 3 for XYZ).
        coord_dimension -> Integer,
        /// Spatial reference system identifier (SRID) of the geography column.
        srid -> Integer,
        /// Type of geography stored in the column (e.g., `POINT`, `LINESTRING`, `POLYGON`).
        r#type -> Text,
    }
}

table! {
    /// `pg_stat_statements` — a statistics view provided by the `pg_stat_statements` extension.
    /// Contains one row per distinct normalized query, with execution counts, planning/execution times,
    /// block I/O, and other performance statistics.
    pg_stat_statements (userid, dbid, queryid) {
        /// OID of the role that executed the statement.
        userid -> diesel::sql_types::Oid,
        /// OID of the database in which the statement was executed.
        dbid -> diesel::sql_types::Oid,
        /// `true` if this entry tracks only top-level statements (not nested ones).
        toplevel -> Bool,
        /// Internal identifier of the query (normalized query hash).
        queryid -> BigInt,
        /// Text of the normalized query (constants replaced with placeholders).
        query -> Text,
        /// Number of times the statement was executed.
        calls -> BigInt,
        /// Number of times the statement was planned.
        plans -> BigInt,
        /// Total time spent planning the statement, in milliseconds.
        total_plan_time -> Double,
        /// Minimum time spent planning, in milliseconds.
        min_plan_time -> Double,
        /// Maximum time spent planning, in milliseconds.
        max_plan_time -> Double,
        /// Mean time spent planning, in milliseconds.
        mean_plan_time -> Double,
        /// Population standard deviation of planning time, in milliseconds.
        stddev_plan_time -> Double,
        /// Total time spent executing the statement, in milliseconds.
        total_exec_time -> Double,
        /// Minimum time spent executing, in milliseconds.
        min_exec_time -> Double,
        /// Maximum time spent executing, in milliseconds.
        max_exec_time -> Double,
        /// Mean time spent executing, in milliseconds.
        mean_exec_time -> Double,
        /// Population standard deviation of execution time, in milliseconds.
        stddev_exec_time -> Double,
        /// Total number of rows retrieved or affected by the statement.
        rows -> BigInt,
        /// Number of shared block cache hits during execution.
        shared_blks_hit -> BigInt,
        /// Number of shared blocks read from disk.
        shared_blks_read -> BigInt,
        /// Number of shared blocks dirtied (modified but not written).
        shared_blks_dirtied -> BigInt,
        /// Number of shared blocks written to disk.
        shared_blks_written -> BigInt,
        /// Number of local block cache hits during execution.
        local_blks_hit -> BigInt,
        /// Number of local blocks read from disk.
        local_blks_read -> BigInt,
        /// Number of local blocks dirtied.
        local_blks_dirtied -> BigInt,
        /// Number of local blocks written to disk.
        local_blks_written -> BigInt,
        /// Number of temporary blocks read from disk.
        temp_blks_read -> BigInt,
        /// Number of temporary blocks written to disk.
        temp_blks_written -> BigInt,
        /// Time spent reading shared blocks, in milliseconds.
        shared_blk_read_time -> Double,
        /// Time spent writing shared blocks, in milliseconds.
        shared_blk_write_time -> Double,
        /// Time spent reading local blocks, in milliseconds.
        local_blk_read_time -> Double,
        /// Time spent writing local blocks, in milliseconds.
        local_blk_write_time -> Double,
    }
}

allow_tables_to_appear_in_same_query!(
    table_constraints,
    key_column_usage,
    tables,
    constraint_column_usage,
    constraint_table_usage,
    columns,
    referential_constraints
);
allow_tables_to_appear_in_same_query!(
    columns,
    pg_type,
    pg_attribute,
    pg_class,
    pg_namespace,
    pg_index
);
allow_tables_to_appear_in_same_query!(check_constraints, table_constraints);
allow_tables_to_appear_in_same_query!(pg_trigger, pg_namespace);
allow_tables_to_appear_in_same_query!(pg_trigger, pg_class);
allow_tables_to_appear_in_same_query!(tables, pg_class);
allow_tables_to_appear_in_same_query!(
    pg_operator,
    pg_constraint,
    pg_depend,
    pg_proc,
    pg_enum,
    pg_extension,
    pg_type,
    tables
);
allow_tables_to_appear_in_same_query!(pg_namespace, pg_proc);
allow_tables_to_appear_in_same_query!(pg_extension, pg_namespace);
allow_tables_to_appear_in_same_query!(check_constraints, constraint_column_usage);
allow_tables_to_appear_in_same_query!(pg_constraint, pg_namespace);
