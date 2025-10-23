//! Diesel schema definitions for the SQL standard `information_schema` views.
//!
//! The `information_schema` is a set of ANSI SQL standard views that provide
//! metadata about database objects in a database-independent way. This module
//! contains Diesel `table!`` macro definitions for all accessible
//! information_schema views in PostgreSQL.
//!
//! ## Covered Views
//!
//! This module includes schemas for views covering:
//! - Tables, columns, and constraints
//! - Domains, user-defined types, and character sets
//! - Views, routines (functions/procedures), and triggers
//! - Privileges and security settings
//! - Foreign data wrappers and foreign tables
//! - Roles and authorization information
//! - SQL features and implementation details
//!
//! Each view is defined in its own submodule with complete column
//! documentation.

pub mod administrable_role_authorizations;
pub mod applicable_roles;
pub mod attributes;
pub mod character_sets;
pub mod check_constraint_routine_usage;
pub mod check_constraints;
pub mod collation_character_set_applicability;
pub mod collations;
pub mod column_column_usage;
pub mod column_domain_usage;
pub mod column_options;
pub mod column_privileges;
pub mod column_udt_usage;
pub mod columns;
pub mod constraint_column_usage;
pub mod constraint_table_usage;
pub mod data_type_privileges;
pub mod domain_constraints;
pub mod domain_udt_usage;
pub mod domains;
pub mod element_types;
pub mod enabled_roles;
pub mod foreign_data_wrapper_options;
pub mod foreign_data_wrappers;
pub mod foreign_server_options;
pub mod foreign_servers;
pub mod foreign_table_options;
pub mod foreign_tables;
pub mod information_schema_catalog_name;
pub mod key_column_usage;
pub mod parameters;
pub mod referential_constraints;
pub mod role_column_grants;
pub mod role_routine_grants;
pub mod role_table_grants;
pub mod role_udt_grants;
pub mod role_usage_grants;
pub mod routine_column_usage;
pub mod routine_privileges;
pub mod routine_routine_usage;
pub mod routine_sequence_usage;
pub mod routine_table_usage;
pub mod routines;
pub mod schemata;
pub mod sequences;
pub mod sql_features;
pub mod sql_implementation_info;
pub mod sql_parts;
pub mod sql_sizing;
pub mod table_constraints;
pub mod table_privileges;
pub mod tables;
pub mod transforms;
pub mod triggered_update_columns;
pub mod triggers;
pub mod udt_privileges;
pub mod usage_privileges;
pub mod user_defined_types;
pub mod user_mapping_options;
pub mod user_mappings;
pub mod view_column_usage;
pub mod view_routine_usage;
pub mod view_table_usage;
pub mod views;
