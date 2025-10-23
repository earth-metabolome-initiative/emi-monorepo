//! Diesel schema definitions for PostgreSQL's `pg_catalog` system tables.
//!
//! The `pg_catalog` schema contains PostgreSQL's system catalog tables and
//! views, which store all metadata about database objects, statistics, and
//! configuration. This module provides Diesel `table!` definitions for these
//! system tables.
//!
//! ## Covered Tables and Views
//!
//! This module includes schemas for:
//! - Core catalog tables: `pg_class`, `pg_attribute`, `pg_type`,
//!   `pg_namespace`, etc.
//! - Constraint and index tables: `pg_constraint`, `pg_index`, etc.
//! - Function and procedure tables: `pg_proc`, `pg_aggregate`, etc.
//! - Statistics views: `pg_stat_*` family for monitoring and statistics
//! - Configuration and settings views: `pg_settings`, `pg_file_settings`, etc.
//! - Replication and publication tables
//! - Extension and access method tables
//!
//! Each table/view is defined in its own submodule with complete column
//! documentation.

pub mod pg_aggregate;
pub mod pg_am;
pub mod pg_amop;
pub mod pg_amproc;
pub mod pg_attrdef;
pub mod pg_attribute;
pub mod pg_auth_members;
pub mod pg_authid;
pub mod pg_available_extension_versions;
pub mod pg_available_extensions;
pub mod pg_backend_memory_contexts;
pub mod pg_cast;
pub mod pg_class;
pub mod pg_collation;
pub mod pg_config;
pub mod pg_constraint;
pub mod pg_conversion;
pub mod pg_cursors;
pub mod pg_database;
pub mod pg_db_role_setting;
pub mod pg_default_acl;
pub mod pg_depend;
pub mod pg_description;
pub mod pg_enum;
pub mod pg_event_trigger;
pub mod pg_extension;
pub mod pg_file_settings;
pub mod pg_foreign_data_wrapper;
pub mod pg_foreign_server;
pub mod pg_foreign_table;
pub mod pg_group;
pub mod pg_hba_file_rules;
pub mod pg_ident_file_mappings;
pub mod pg_index;
pub mod pg_indexes;
pub mod pg_inherits;
pub mod pg_init_privs;
pub mod pg_language;
pub mod pg_largeobject;
pub mod pg_largeobject_metadata;
pub mod pg_locks;
pub mod pg_matviews;
pub mod pg_namespace;
pub mod pg_opclass;
pub mod pg_operator;
pub mod pg_opfamily;
pub mod pg_parameter_acl;
pub mod pg_partitioned_table;
pub mod pg_policies;
pub mod pg_policy;
pub mod pg_prepared_statements;
pub mod pg_prepared_xacts;
pub mod pg_proc;
pub mod pg_publication;
pub mod pg_publication_namespace;
pub mod pg_publication_rel;
pub mod pg_publication_tables;
pub mod pg_range;
pub mod pg_replication_origin;
pub mod pg_replication_origin_status;
pub mod pg_replication_slots;
pub mod pg_rewrite;
pub mod pg_roles;
pub mod pg_rules;
pub mod pg_seclabel;
pub mod pg_seclabels;
pub mod pg_sequence;
pub mod pg_sequences;
pub mod pg_settings;
pub mod pg_shadow;
pub mod pg_shdepend;
pub mod pg_shdescription;
pub mod pg_shmem_allocations;
pub mod pg_shseclabel;
pub mod pg_stat_activity;
pub mod pg_stat_all_indexes;
pub mod pg_stat_all_tables;
pub mod pg_stat_archiver;
pub mod pg_stat_bgwriter;
pub mod pg_stat_checkpointer;
pub mod pg_stat_database;
pub mod pg_stat_database_conflicts;
pub mod pg_stat_gssapi;
pub mod pg_stat_io;
pub mod pg_stat_progress_analyze;
pub mod pg_stat_progress_basebackup;
pub mod pg_stat_progress_cluster;
pub mod pg_stat_progress_copy;
pub mod pg_stat_progress_create_index;
pub mod pg_stat_progress_vacuum;
pub mod pg_stat_recovery_prefetch;
pub mod pg_stat_replication;
pub mod pg_stat_replication_slots;
pub mod pg_stat_slru;
pub mod pg_stat_ssl;
pub mod pg_stat_statements;
pub mod pg_stat_subscription;
pub mod pg_stat_subscription_stats;
pub mod pg_stat_sys_indexes;
pub mod pg_stat_sys_tables;
pub mod pg_stat_user_functions;
pub mod pg_stat_user_indexes;
pub mod pg_stat_user_tables;
pub mod pg_stat_wal;
pub mod pg_stat_wal_receiver;
pub mod pg_stat_xact_all_tables;
pub mod pg_stat_xact_sys_tables;
pub mod pg_stat_xact_user_functions;
pub mod pg_stat_xact_user_tables;
pub mod pg_statio_all_indexes;
pub mod pg_statio_all_sequences;
pub mod pg_statio_all_tables;
pub mod pg_statio_sys_indexes;
pub mod pg_statio_sys_sequences;
pub mod pg_statio_sys_tables;
pub mod pg_statio_user_indexes;
pub mod pg_statio_user_sequences;
pub mod pg_statio_user_tables;
pub mod pg_statistic;
pub mod pg_statistic_ext;
pub mod pg_statistic_ext_data;
pub mod pg_stats;
pub mod pg_stats_ext;
pub mod pg_stats_ext_exprs;
pub mod pg_subscription;
pub mod pg_subscription_rel;
pub mod pg_tables;
pub mod pg_tablespace;
pub mod pg_timezone_abbrevs;
pub mod pg_timezone_names;
pub mod pg_transform;
pub mod pg_trigger;
pub mod pg_ts_config;
pub mod pg_ts_config_map;
pub mod pg_ts_dict;
pub mod pg_ts_parser;
pub mod pg_ts_template;
pub mod pg_type;
pub mod pg_user;
pub mod pg_user_mapping;
pub mod pg_user_mappings;
pub mod pg_views;
pub mod pg_wait_events;
