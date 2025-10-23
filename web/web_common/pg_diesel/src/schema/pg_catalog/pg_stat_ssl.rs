//! Submodule for the `pg_catalog.pg_stat_ssl` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_ssl` — view showing SSL connection information.
    /// Each row represents one connection showing SSL-related information for that connection.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_ssl (pid) {
        /// Process ID of the backend.
        pid -> Nullable<Integer>,
        /// Whether SSL is used on this connection.
        ssl -> Nullable<Bool>,
        /// Version of SSL in use.
        version -> Nullable<Text>,
        /// Name of SSL cipher in use.
        cipher -> Nullable<Text>,
        /// Number of bits in the encryption algorithm.
        bits -> Nullable<Integer>,
        /// Distinguished name (DN) of the client certificate.
        client_dn -> Nullable<Text>,
        /// Serial number of the client certificate.
        client_serial -> Nullable<Double>,
        /// DN of the issuer of the client certificate.
        issuer_dn -> Nullable<Text>,
    }
}
