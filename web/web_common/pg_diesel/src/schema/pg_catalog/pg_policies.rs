//! Submodule for the `pg_catalog.pg_policies` view schema.

diesel::table! {
    /// `pg_catalog.pg_policies` — view showing information about row-level security policies.
    /// Each row represents a policy that restricts which rows can be accessed or modified.
    /// Uses `policyname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_policies (policyname) {
        /// Name of the schema containing the table.
        schemaname -> Nullable<Text>,
        /// Name of the table the policy applies to.
        tablename -> Nullable<Text>,
        /// Name of the policy.
        policyname -> Nullable<Text>,
        /// Policy type: "PERMISSIVE" or "RESTRICTIVE".
        permissive -> Nullable<Text>,
        /// Roles to which the policy applies.
        roles -> Nullable<Array<Text>>,
        /// Command type: "ALL", "SELECT", "INSERT", "UPDATE", or "DELETE".
        cmd -> Nullable<Text>,
        /// Expression defining which rows the policy applies to (USING clause).
        qual -> Nullable<Text>,
        /// Expression defining which rows can be created/updated (WITH CHECK clause).
        with_check -> Nullable<Text>,
    }
}
