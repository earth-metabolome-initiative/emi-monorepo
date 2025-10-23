//! Submodule for the `pg_catalog.pg_database` table schema.

diesel::table! {
    /// `pg_catalog.pg_database` — system catalog containing one row per database.
    /// Stores metadata about databases including encoding, locale, connection limits,
    /// and access control lists.
    pg_catalog.pg_database (oid) {
        /// OID of the database.
        oid -> Oid,
        /// Name of the database.
        datname -> Text,
        /// OID of the role that owns the database.
        datdba -> Oid,
        /// Character encoding for this database (encoding ID from pg_encoding).
        encoding -> Integer,
        /// Locale provider: 'c' (libc), 'i' (icu), or 'd' (database default).
        datlocprovider -> Text,
        /// `true` if this is a template database (can be cloned with CREATE DATABASE).
        datistemplate -> Bool,
        /// `true` if connections to this database are allowed.
        datallowconn -> Bool,
        /// `true` if login events are logged for this database.
        dathasloginevt -> Bool,
        /// Maximum number of concurrent connections (-1 = no limit).
        datconnlimit -> Integer,
        /// All transaction IDs before this one have been replaced with a permanent ID.
        datfrozenxid -> Oid,
        /// All multixact IDs before this one have been replaced with a permanent ID.
        datminmxid -> Oid,
        /// Default tablespace for this database.
        dattablespace -> Oid,
        /// LC_COLLATE setting for this database.
        datcollate -> Text,
        /// LC_CTYPE setting for this database.
        datctype -> Text,
        /// Locale name if using ICU provider.
        datlocale -> Nullable<Text>,
        /// ICU collation rules if using ICU provider.
        daticurules -> Nullable<Text>,
        /// Version of the collation.
        datcollversion -> Nullable<Text>,
        /// Access privileges (ACL) for the database.
        datacl -> Nullable<Array<Text>>,
    }
}
