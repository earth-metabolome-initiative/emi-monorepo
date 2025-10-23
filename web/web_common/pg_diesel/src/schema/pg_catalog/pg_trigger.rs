//! Submodule for the `pg_trigger` table schema.

diesel::table! {
    /// `pg_trigger` — system catalog containing one row for each trigger in the database.
    /// Stores metadata about trigger definitions, including their name, function, firing conditions,
    /// and relation associations.
    pg_catalog.pg_trigger (oid) {
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
        /// Predicate expression (WHEN clause) for the trigger, or `NULL` if none.
        tgqual -> Nullable<Text>,
    }
}

use super::pg_namespace::pg_namespace;
diesel::allow_tables_to_appear_in_same_query!(pg_trigger, pg_namespace);
