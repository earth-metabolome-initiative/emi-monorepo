//! Submodule for the `pg_catalog.pg_policy` table schema.

diesel::table! {
    /// `pg_catalog.pg_policy` — system catalog containing row-level security policies.
    /// Each row represents a policy defined on a table.
    pg_catalog.pg_policy (oid) {
        /// OID of the policy.
        oid -> Oid,
        /// Name of the policy.
        polname -> Text,
        /// OID of the table the policy applies to.
        polrelid -> Oid,
        /// Command type: '*' (all), 'r' (select), 'a' (insert), 'w' (update), 'd' (delete).
        polcmd -> Text,
        /// `true` if the policy is permissive, `false` if restrictive.
        polpermissive -> Bool,
        /// Array of role OIDs to which this policy applies.
        polroles -> Array<Oid>,
        /// Expression tree for the USING clause (null if none).
        polqual -> Nullable<Text>,
        /// Expression tree for the WITH CHECK clause (null if none).
        polwithcheck -> Nullable<Text>,
    }
}
