//! Submodule for the `pg_catalog.pg_stat_gssapi` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_gssapi` — view showing GSSAPI authentication information.
    /// Each row represents one connection showing GSSAPI-related information for that connection.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_gssapi (pid) {
        /// Process ID of the backend.
        pid -> Nullable<Integer>,
        /// Whether GSSAPI authentication was used for this connection.
        gss_authenticated -> Nullable<Bool>,
        /// Principal used to authenticate this connection.
        principal -> Nullable<Text>,
        /// Whether GSSAPI encryption is in use for this connection.
        encrypted -> Nullable<Bool>,
        /// Whether GSSAPI credentials were delegated for this connection.
        credentials_delegated -> Nullable<Bool>,
    }
}
