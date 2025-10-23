//! Model structs for the SQL standard `information_schema` views.
//!
//! This module contains Diesel-queryable structs representing rows from the
//! ANSI SQL standard `information_schema` views. These views provide portable,
//! database-independent metadata about database objects.
//!
//! ## Key Models
//!
//! - [`Table`]: Represents a row from `information_schema.tables`
//! - [`Column`]: Represents a row from `information_schema.columns`
//! - [`CheckConstraint`]: Represents a row from
//!   `information_schema.check_constraints`
//! - [`KeyColumnUsage`]: Represents foreign key information from
//!   `information_schema.key_column_usage`
//! - [`ReferentialConstraint`]: Represents foreign key constraints from
//!   `information_schema.referential_constraints`
//!
//! All structs implement `Queryable`, `QueryableByName`, and `Selectable` from
//! Diesel, and many implement additional traits from the `sql_traits` crate for
//! database introspection.

mod administrable_role_authorizations;
mod applicable_roles;
mod attribute;
mod character_set;
mod check_constraint;
mod check_constraint_routine_usage;
mod collation;
mod collation_character_set_applicability;
mod column;
mod column_column_usage;
mod column_domain_usage;
mod column_options;
mod column_privilege;
mod column_udt_usage;
mod constraint_column_usage;
mod constraint_table_usage;
mod data_type_privilege;
mod domain;
mod domain_constraint;
mod domain_udt_usage;
mod element_types;
mod enabled_roles;
mod foreign_data_wrapper_options;
mod foreign_data_wrappers;
mod foreign_server_options;
mod foreign_servers;
mod foreign_table_options;
mod foreign_tables;
mod information_schema_catalog_name;
mod key_column_usage;
mod parameters;
mod referential_constraint;
mod role_column_grants;
mod role_routine_grants;
mod role_table_grants;
mod role_udt_grants;
mod role_usage_grants;
mod routine_column_usage;
mod routine_privileges;
mod routine_routine_usage;
mod routine_sequence_usage;
mod routine_table_usage;
mod routines;
mod schemata;
mod sequences;
mod sql_features;
mod sql_implementation_info;
mod sql_parts;
mod sql_sizing;
mod table;
mod table_constraint;
mod table_privileges;
mod transforms;
mod triggered_update_columns;
mod triggers;
mod udt_privileges;
mod usage_privileges;
mod user_defined_types;
mod user_mapping_options;
mod user_mappings;
mod view_column_usage;
mod view_routine_usage;
mod view_table_usage;
mod views;

pub use administrable_role_authorizations::AdministrableRoleAuthorizations;
pub use applicable_roles::ApplicableRoles;
pub use attribute::Attribute;
pub use character_set::CharacterSet;
pub use check_constraint::CheckConstraint;
pub use check_constraint_routine_usage::CheckConstraintRoutineUsage;
pub use collation::Collation;
pub use collation_character_set_applicability::CollationCharacterSetApplicability;
pub use column::Column;
pub use column_column_usage::ColumnColumnUsage;
pub use column_domain_usage::ColumnDomainUsage;
pub use column_options::ColumnOptions;
pub use column_privilege::ColumnPrivilege;
pub use column_udt_usage::ColumnUdtUsage;
pub use constraint_column_usage::ConstraintColumnUsage;
pub use constraint_table_usage::ConstraintTableUsage;
pub use data_type_privilege::DataTypePrivilege;
pub use domain::Domain;
pub use domain_constraint::DomainConstraint;
pub use domain_udt_usage::DomainUdtUsage;
pub use element_types::ElementTypes;
pub use enabled_roles::EnabledRoles;
pub use foreign_data_wrapper_options::ForeignDataWrapperOptions;
pub use foreign_data_wrappers::ForeignDataWrappers;
pub use foreign_server_options::ForeignServerOptions;
pub use foreign_servers::ForeignServers;
pub use foreign_table_options::ForeignTableOptions;
pub use foreign_tables::ForeignTables;
pub use information_schema_catalog_name::InformationSchemaCatalogName;
pub use key_column_usage::KeyColumnUsage;
pub use parameters::Parameters;
pub use referential_constraint::ReferentialConstraint;
pub use role_column_grants::RoleColumnGrants;
pub use role_routine_grants::RoleRoutineGrants;
pub use role_table_grants::RoleTableGrants;
pub use role_udt_grants::RoleUdtGrants;
pub use role_usage_grants::RoleUsageGrants;
pub use routine_column_usage::RoutineColumnUsage;
pub use routine_privileges::RoutinePrivileges;
pub use routine_routine_usage::RoutineRoutineUsage;
pub use routine_sequence_usage::RoutineSequenceUsage;
pub use routine_table_usage::RoutineTableUsage;
pub use routines::Routines;
pub use schemata::Schemata;
pub use sequences::Sequences;
pub use sql_features::SqlFeatures;
pub use sql_implementation_info::SqlImplementationInfo;
pub use sql_parts::SqlParts;
pub use sql_sizing::SqlSizing;
pub use table::Table;
pub use table_constraint::TableConstraint;
pub use table_privileges::TablePrivileges;
pub use transforms::Transforms;
pub use triggered_update_columns::TriggeredUpdateColumns;
pub use triggers::Triggers;
pub use udt_privileges::UdtPrivileges;
pub use usage_privileges::UsagePrivileges;
pub use user_defined_types::UserDefinedTypes;
pub use user_mapping_options::UserMappingOptions;
pub use user_mappings::UserMappings;
pub use view_column_usage::ViewColumnUsage;
pub use view_routine_usage::ViewRoutineUsage;
pub use view_table_usage::ViewTableUsage;
pub use views::Views;
