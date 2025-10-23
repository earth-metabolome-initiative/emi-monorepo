//! Model structs for PostgreSQL's `pg_catalog` system tables.
//!
//! This module contains Diesel-queryable structs representing rows from
//! PostgreSQL's system catalog tables in the `pg_catalog` schema. These tables
//! store all metadata about database objects, configuration, and runtime
//! statistics.
//!
//! ## Key Models
//!
//! ### Core Catalog Tables
//! - [`PGClass`]: Represents tables, indexes, sequences, etc. from
//!   `pg_catalog.pg_class`
//! - [`PgAttribute`]: Represents table columns from `pg_catalog.pg_attribute`
//! - [`PgType`]: Represents data types from `pg_catalog.pg_type`
//! - [`PgProc`]: Represents functions and procedures from `pg_catalog.pg_proc`
//! - [`PgConstraint`]: Represents constraints from `pg_catalog.pg_constraint`
//! - [`PgIndex`]: Represents indexes from `pg_catalog.pg_index`
//!
//! ### Statistics and Monitoring Views
//! - `PgStat*` family: Runtime statistics views (activity, tables, indexes,
//!   etc.)
//! - `PgStatio*` family: I/O statistics views
//!
//! ### Configuration and Extensions
//! - [`PgSetting`]: Configuration settings from `pg_catalog.pg_settings`
//! - [`PgExtension`]: Installed extensions from `pg_catalog.pg_extension`
//!
//! All structs implement `Queryable`, `QueryableByName`, and `Selectable` from
//! Diesel. Many also implement traits like [`HasOid`](crate::traits::HasOid)
//! for types with Object Identifiers.

mod pg_aggregate;
mod pg_am;
mod pg_amop;
mod pg_amproc;
mod pg_attrdef;
mod pg_attribute;
mod pg_auth_members;
mod pg_authid;
mod pg_available_extension_versions;
mod pg_available_extensions;
mod pg_backend_memory_contexts;
mod pg_cast;
mod pg_class;
mod pg_collation;
mod pg_config;
mod pg_constraint;
mod pg_conversion;
mod pg_cursor;
mod pg_database;
mod pg_db_role_setting;
mod pg_default_acl;
mod pg_depend;
mod pg_description;
mod pg_enum;
mod pg_event_trigger;
mod pg_extension;
mod pg_file_setting;
mod pg_foreign_data_wrapper;
mod pg_foreign_server;
mod pg_foreign_table;
mod pg_group;
mod pg_hba_file_rule;
mod pg_ident_file_mapping;
pub(crate) mod pg_index;
mod pg_inherit;
mod pg_init_priv;
mod pg_language;
mod pg_largeobject;
mod pg_largeobject_metadata;
mod pg_lock;
mod pg_matview;
mod pg_opclass;
mod pg_operator;
mod pg_opfamily;
mod pg_parameter_acl;
mod pg_partitioned_table;
mod pg_policy;
mod pg_policy_table;
mod pg_prepared_statement;
mod pg_prepared_xact;
mod pg_proc;
mod pg_publication;
mod pg_publication_namespace;
mod pg_publication_rel;
mod pg_publication_table;
mod pg_range;
mod pg_replication_origin;
mod pg_replication_origin_status;
mod pg_replication_slot;
mod pg_rewrite;
mod pg_role;
mod pg_rule;
mod pg_seclabel;
mod pg_seclabel_view;
mod pg_sequence;
mod pg_sequence_view;
mod pg_setting;
mod pg_shadow;
mod pg_shdepend;
mod pg_shdescription;
mod pg_shmem_allocation;
mod pg_shseclabel;
mod pg_stat;
mod pg_stat_activity;
mod pg_stat_all_index;
mod pg_stat_all_table;
mod pg_stat_archiver;
mod pg_stat_bgwriter;
mod pg_stat_checkpointer;
mod pg_stat_database;
mod pg_stat_database_conflict;
mod pg_stat_gssapi;
mod pg_stat_io;
mod pg_stat_progress_analyze;
mod pg_stat_progress_basebackup;
mod pg_stat_progress_cluster;
mod pg_stat_progress_copy;
mod pg_stat_progress_create_index;
mod pg_stat_progress_vacuum;
mod pg_stat_recovery_prefetch;
mod pg_stat_replication;
mod pg_stat_replication_slot;
mod pg_stat_slru;
mod pg_stat_ssl;
mod pg_stat_statements;
mod pg_stat_subscription;
mod pg_stat_subscription_stat;
mod pg_stat_sys_index;
mod pg_stat_sys_table;
mod pg_stat_user_function;
mod pg_stat_user_index;
mod pg_stat_user_table;
mod pg_stat_wal;
mod pg_stat_wal_receiver;
mod pg_stat_xact_all_table;
mod pg_stat_xact_sys_table;
mod pg_stat_xact_user_function;
mod pg_stat_xact_user_table;
mod pg_statio_all_index;
mod pg_statio_all_sequence;
mod pg_statio_all_table;
mod pg_statio_sys_index;
mod pg_statio_sys_sequence;
mod pg_statio_sys_table;
mod pg_statio_user_index;
mod pg_statio_user_sequence;
mod pg_statio_user_table;
mod pg_statistic;
mod pg_statistic_ext;
mod pg_statistic_ext_datum;
mod pg_stats_ext;
mod pg_stats_ext_expr;
mod pg_subscription;
mod pg_subscription_rel;
mod pg_table;
mod pg_tablespace;
mod pg_timezone_abbrev;
mod pg_timezone_name;
mod pg_transform;
mod pg_trigger;
mod pg_ts_config;
mod pg_ts_config_map;
mod pg_ts_dict;
mod pg_ts_parser;
mod pg_ts_template;
mod pg_type;
mod pg_user;
mod pg_user_mapping;
mod pg_user_mappings;
mod pg_view;
mod pg_wait_event;

pub use pg_aggregate::PgAggregate;
pub use pg_am::PgAm;
pub use pg_amop::PgAmop;
pub use pg_amproc::PgAmproc;
pub use pg_attrdef::PgAttrdef;
pub use pg_attribute::PgAttribute;
pub use pg_auth_members::PgAuthMembers;
pub use pg_authid::PgAuthid;
pub use pg_available_extension_versions::PgAvailableExtensionVersions;
pub use pg_available_extensions::PgAvailableExtensions;
pub use pg_backend_memory_contexts::PgBackendMemoryContexts;
pub use pg_cast::PgCast;
pub use pg_class::PGClass;
pub use pg_collation::PgCollation;
pub use pg_config::PgConfig;
pub use pg_constraint::PgConstraint;
pub use pg_conversion::PgConversion;
pub use pg_cursor::PgCursor;
pub use pg_database::PgDatabase;
pub use pg_db_role_setting::PgDbRoleSetting;
pub use pg_default_acl::PgDefaultAcl;
pub use pg_depend::PgDepend;
pub use pg_description::PgDescription;
pub use pg_enum::PgEnum;
pub use pg_event_trigger::PgEventTrigger;
pub use pg_extension::PgExtension;
pub use pg_file_setting::PgFileSetting;
pub use pg_foreign_data_wrapper::PgForeignDataWrapper;
pub use pg_foreign_server::PgForeignServer;
pub use pg_foreign_table::PgForeignTable;
pub use pg_group::PgGroup;
pub use pg_hba_file_rule::PgHbaFileRule;
pub use pg_ident_file_mapping::PgIdentFileMapping;
pub use pg_index::PgIndex;
pub use pg_inherit::PgInherit;
pub use pg_init_priv::PgInitPriv;
pub use pg_language::PgLanguage;
pub use pg_largeobject::PgLargeobject;
pub use pg_largeobject_metadata::PgLargeobjectMetadatum;
pub use pg_lock::PgLock;
pub use pg_matview::PgMatview;
pub use pg_opclass::PgOpclass;
pub use pg_operator::PgOperator;
pub use pg_opfamily::PgOpfamily;
pub use pg_parameter_acl::PgParameterAcl;
pub use pg_partitioned_table::PgPartitionedTable;
pub use pg_policy::PgPolicy;
pub use pg_policy_table::PgPolicyTable;
pub use pg_prepared_statement::PgPreparedStatement;
pub use pg_prepared_xact::PgPreparedXact;
pub use pg_proc::PgProc;
pub use pg_publication::PgPublication;
pub use pg_publication_namespace::PgPublicationNamespace;
pub use pg_publication_rel::PgPublicationRel;
pub use pg_publication_table::PgPublicationTable;
pub use pg_range::PgRange;
pub use pg_replication_origin::PgReplicationOrigin;
pub use pg_replication_origin_status::PgReplicationOriginStatus;
pub use pg_replication_slot::PgReplicationSlot;
pub use pg_rewrite::PgRewrite;
pub use pg_role::PgRole;
pub use pg_rule::PgRule;
pub use pg_seclabel::PgSeclabel;
pub use pg_seclabel_view::PgSeclabelView;
pub use pg_sequence::PgSequence;
pub use pg_sequence_view::PgSequenceView;
pub use pg_setting::PgSetting;
pub use pg_shadow::PgShadow;
pub use pg_shdepend::PgShdepend;
pub use pg_shdescription::PgShdescription;
pub use pg_shmem_allocation::PgShmemAllocation;
pub use pg_shseclabel::PgShseclabel;
pub use pg_stat::PgStat;
pub use pg_stat_activity::PgStatActivity;
pub use pg_stat_all_index::PgStatAllIndex;
pub use pg_stat_all_table::PgStatAllTable;
pub use pg_stat_archiver::PgStatArchiver;
pub use pg_stat_bgwriter::PgStatBgwriter;
pub use pg_stat_checkpointer::PgStatCheckpointer;
pub use pg_stat_database::PgStatDatabase;
pub use pg_stat_database_conflict::PgStatDatabaseConflict;
pub use pg_stat_gssapi::PgStatGssapi;
pub use pg_stat_io::PgStatIo;
pub use pg_stat_progress_analyze::PgStatProgressAnalyze;
pub use pg_stat_progress_basebackup::PgStatProgressBasebackup;
pub use pg_stat_progress_cluster::PgStatProgressCluster;
pub use pg_stat_progress_copy::PgStatProgressCopy;
pub use pg_stat_progress_create_index::PgStatProgressCreateIndex;
pub use pg_stat_progress_vacuum::PgStatProgressVacuum;
pub use pg_stat_recovery_prefetch::PgStatRecoveryPrefetch;
pub use pg_stat_replication::PgStatReplication;
pub use pg_stat_replication_slot::PgStatReplicationSlot;
pub use pg_stat_slru::PgStatSlru;
pub use pg_stat_ssl::PgStatSsl;
pub use pg_stat_statements::PgStatStatement;
pub use pg_stat_subscription::PgStatSubscription;
pub use pg_stat_subscription_stat::PgStatSubscriptionStat;
pub use pg_stat_sys_index::PgStatSysIndex;
pub use pg_stat_sys_table::PgStatSysTable;
pub use pg_stat_user_function::PgStatUserFunction;
pub use pg_stat_user_index::PgStatUserIndex;
pub use pg_stat_user_table::PgStatUserTable;
pub use pg_stat_wal::PgStatWal;
pub use pg_stat_wal_receiver::PgStatWalReceiver;
pub use pg_stat_xact_all_table::PgStatXactAllTable;
pub use pg_stat_xact_sys_table::PgStatXactSysTable;
pub use pg_stat_xact_user_function::PgStatXactUserFunction;
pub use pg_stat_xact_user_table::PgStatXactUserTable;
pub use pg_statio_all_index::PgStatioAllIndex;
pub use pg_statio_all_sequence::PgStatioAllSequence;
pub use pg_statio_all_table::PgStatioAllTable;
pub use pg_statio_sys_index::PgStatioSysIndex;
pub use pg_statio_sys_sequence::PgStatioSysSequence;
pub use pg_statio_sys_table::PgStatioSysTable;
pub use pg_statio_user_index::PgStatioUserIndex;
pub use pg_statio_user_sequence::PgStatioUserSequence;
pub use pg_statio_user_table::PgStatioUserTable;
pub use pg_statistic::PgStatistic;
pub use pg_statistic_ext::PgStatisticExt;
pub use pg_statistic_ext_datum::PgStatisticExtDatum;
pub use pg_stats_ext::PgStatsExt;
pub use pg_stats_ext_expr::PgStatsExtExpr;
pub use pg_subscription::PgSubscription;
pub use pg_subscription_rel::PgSubscriptionRel;
pub use pg_table::PgTable;
pub use pg_tablespace::PgTablespace;
pub use pg_timezone_abbrev::PgTimezoneAbbrev;
pub use pg_timezone_name::PgTimezoneName;
pub use pg_transform::PgTransform;
pub use pg_trigger::PgTrigger;
pub use pg_ts_config::PgTsConfig;
pub use pg_ts_config_map::PgTsConfigMap;
pub use pg_ts_dict::PgTsDict;
pub use pg_ts_parser::PgTsParser;
pub use pg_ts_template::PgTsTemplate;
pub use pg_type::PgType;
pub use pg_user::PgUser;
pub use pg_user_mapping::PgUserMapping;
pub use pg_user_mappings::PgUserMappings;
pub use pg_view::PgView;
pub use pg_wait_event::PgWaitEvent;
