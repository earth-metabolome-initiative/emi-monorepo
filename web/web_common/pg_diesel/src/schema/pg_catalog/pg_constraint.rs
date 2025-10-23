//! Submodule for the `pg_constraint` table schema.

diesel::table! {
    /// `pg_constraint` — system catalog containing one row for each table, domain, or index constraint.
    /// Stores metadata about primary keys, foreign keys, unique constraints, check constraints,
    /// exclusion constraints, and domain constraints.
    pg_catalog.pg_constraint (oid, conname, connamespace) {
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
        /// Column numbers of the referenced columns in the foreign key's referenced table, or `NULL`.
        confkey -> Nullable<Array<SmallInt>>,
        /// Equality operators for the pairs of constrained columns (foreign key), or `NULL`.
        conpfeqop -> Nullable<Array<Oid>>,
        /// Equality operators for pairs of parent and child columns in table inheritance, or `NULL`.
        conppeqop -> Nullable<Array<Oid>>,
        /// Equality operators for referenced columns in the foreign key's referenced table, or `NULL`.
        conffeqop -> Nullable<Array<Oid>>,
        /// Column numbers in the referencing table that are set in `ON DELETE SET NULL` or `SET DEFAULT`, or `NULL`.
        confdelsetcols -> Nullable<Array<SmallInt>>,
        /// Operator OIDs for exclusion constraints; `NULL` if not applicable.
        conexclop -> Nullable<Array<Oid>>,
        /// Check constraint expression as a human-readable string, or `NULL` if not a check constraint.
        conbin -> Nullable<Text>,
    }
}

use super::pg_depend::pg_depend;
diesel::allow_tables_to_appear_in_same_query!(pg_constraint, pg_depend);

use super::pg_proc::pg_proc;
diesel::allow_tables_to_appear_in_same_query!(pg_constraint, pg_proc);

use super::pg_operator::pg_operator;
diesel::allow_tables_to_appear_in_same_query!(pg_constraint, pg_operator);

use super::pg_namespace::pg_namespace;
diesel::allow_tables_to_appear_in_same_query!(pg_constraint, pg_namespace);
