//! Submodule for the `pg_catalog.pg_hba_file_rules` view schema.

diesel::table! {
    /// `pg_catalog.pg_hba_file_rules` — view showing the contents of the client authentication
    /// configuration file (pg_hba.conf). Each row represents one rule from the file.
    pg_catalog.pg_hba_file_rules (rule_number) {
        /// Rule number (sequence in which rules are processed).
        rule_number -> Nullable<Integer>,
        /// Name of the file containing this rule.
        file_name -> Nullable<Text>,
        /// Line number of this rule within the file.
        line_number -> Nullable<Integer>,
        /// Connection type: 'local', 'host', 'hostssl', 'hostnossl', etc.
        #[sql_name = "type"]
        r#type -> Nullable<Text>,
        /// Array of database names this rule applies to.
        database -> Nullable<Array<Text>>,
        /// Array of user names this rule applies to.
        user_name -> Nullable<Array<Text>>,
        /// IP address or hostname (for network-based rules).
        address -> Nullable<Text>,
        /// Network mask (for IP-based rules).
        netmask -> Nullable<Text>,
        /// Authentication method: 'trust', 'reject', 'md5', 'scram-sha-256', etc.
        auth_method -> Nullable<Text>,
        /// Authentication method options.
        options -> Nullable<Array<Text>>,
        /// Error message if the rule is invalid.
        error -> Nullable<Text>,
    }
}
