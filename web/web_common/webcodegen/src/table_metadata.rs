//! Submodule defining diesel-based structs for Postgres core metadata tables.

use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};

/// Struct defining the `information_schema.tables` table.
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::tables)]
pub struct Table {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_type: String,
    pub self_referencing_column_name: Option<String>,
    pub reference_generation: Option<String>,
    pub user_defined_type_catalog: Option<String>,
    pub user_defined_type_schema: Option<String>,
    pub user_defined_type_name: Option<String>,
    pub is_insertable_into: String,
    pub is_typed: String,
    pub commit_action: Option<String>,
}

impl Table {
    pub fn load_all_tables(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::tables::dsl::*;
        tables.load::<Table>(conn).expect("Error loading tables")
    }

    pub fn load(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Option<Self> {
        use crate::schema::tables;
        let table_schema = table_schema.unwrap_or("public");
        tables::dsl::tables
            .filter(tables::dsl::table_name.eq(table_name))
            .filter(tables::dsl::table_schema.eq(table_schema))
            .filter(tables::dsl::table_catalog.eq(table_catalog))
            .first::<Table>(conn)
            .ok()
    }

    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, DieselError> {
        use crate::schema::columns;
        columns::dsl::columns
            .filter(columns::dsl::table_name.eq(&self.table_name))
            .filter(columns::dsl::table_schema.eq(&self.table_schema))
            .filter(columns::dsl::table_catalog.eq(&self.table_catalog))
            .load::<Column>(conn)
    }
}

/// Struct defining the `information_schema.columns` table.
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::columns)]
pub struct Column {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_position: i32,
    pub column_default: Option<String>,
    pub __is_nullable: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub character_octet_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_precision_radix: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub datetime_precision: Option<i32>,
    pub interval_type: Option<String>,
    pub interval_precision: Option<i32>,
    pub character_set_catalog: Option<String>,
    pub character_set_schema: Option<String>,
    pub character_set_name: Option<String>,
    pub collation_catalog: Option<String>,
    pub collation_schema: Option<String>,
    pub collation_name: Option<String>,
    pub domain_catalog: Option<String>,
    pub domain_schema: Option<String>,
    pub domain_name: Option<String>,
    pub udt_catalog: Option<String>,
    pub udt_schema: Option<String>,
    pub udt_name: Option<String>,
    pub scope_catalog: Option<String>,
    pub scope_schema: Option<String>,
    pub scope_name: Option<String>,
    pub maximum_cardinality: Option<i32>,
    pub dtd_identifier: Option<String>,
    pub is_self_referencing: Option<String>,
    pub is_identity: Option<String>,
    pub identity_generation: Option<String>,
    pub identity_start: Option<String>,
    pub identity_increment: Option<String>,
    pub identity_maximum: Option<String>,
    pub identity_minimum: Option<String>,
    pub identity_cycle: Option<String>,
    pub is_generated: String,
    pub generation_expression: Option<String>,
    pub is_updatable: String,
}

impl Column {
    pub fn load_all_columns(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::columns::dsl::*;
        columns.load::<Column>(conn).expect("Error loading columns")
    }
}

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::table_constraints)]
pub struct TableConstraint {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub constraint_type: String,
    pub is_deferrable: String,
    pub initially_deferred: String,
    pub enforced: String,
    pub nulls_distinct: Option<String>,
}

impl TableConstraint {
    pub fn load_all_table_constraints(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::table_constraints::dsl::*;
        table_constraints
            .load::<TableConstraint>(conn)
            .expect("Error loading table constraints")
    }

    pub fn load_table_constraints(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::table_constraints;
        let table_schema = table_schema.unwrap_or("public");
        table_constraints::dsl::table_constraints
            .filter(table_constraints::dsl::table_name.eq(table_name))
            .filter(table_constraints::dsl::table_schema.eq(table_schema))
            .filter(table_constraints::dsl::table_catalog.eq(table_catalog))
            .load::<TableConstraint>(conn)
            .expect("Error loading table constraints")
    }
}

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::key_column_usage)]
pub struct KeyColumnUsage {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_position: i32,
    pub position_in_unique_constraint: Option<i32>,
}

impl KeyColumnUsage {
    pub fn load_all_key_column_usages(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::key_column_usage::dsl::*;
        key_column_usage
            .load::<KeyColumnUsage>(conn)
            .expect("Error loading key column usages")
    }

    pub fn load_key_column_usages(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::key_column_usage;
        let table_schema = table_schema.unwrap_or("public");
        key_column_usage::dsl::key_column_usage
            .filter(key_column_usage::dsl::table_name.eq(table_name))
            .filter(key_column_usage::dsl::table_schema.eq(table_schema))
            .filter(key_column_usage::dsl::table_catalog.eq(table_catalog))
            .load::<KeyColumnUsage>(conn)
            .expect("Error loading key column usages")
    }
}

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::referential_constraints)]
pub struct ReferentialConstraint {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub unique_constraint_catalog: Option<String>,
    pub unique_constraint_schema: Option<String>,
    pub unique_constraint_name: Option<String>,
    pub match_option: String,
    pub update_rule: String,
    pub delete_rule: String,
}

impl ReferentialConstraint {
    pub fn load_all_referential_constraints(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::referential_constraints::dsl::*;
        referential_constraints
            .load::<ReferentialConstraint>(conn)
            .expect("Error loading referential constraints")
    }

    pub fn load_referential_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::referential_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        referential_constraints::dsl::referential_constraints
            .filter(referential_constraints::dsl::constraint_name.eq(constraint_name))
            .filter(referential_constraints::dsl::constraint_schema.eq(constraint_schema))
            .filter(referential_constraints::dsl::constraint_catalog.eq(constraint_catalog))
            .load::<ReferentialConstraint>(conn)
            .expect("Error loading referential constraints")
    }
}

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::constraint_column_usage)]
pub struct ConstraintColumnUsage {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
}

impl ConstraintColumnUsage {
    pub fn load_all_constraint_column_usages(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::constraint_column_usage::dsl::*;
        constraint_column_usage
            .load::<ConstraintColumnUsage>(conn)
            .expect("Error loading constraint column usages")
    }

    pub fn load_constraint_column_usages(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::constraint_column_usage;
        let constraint_schema = constraint_schema.unwrap_or("public");
        constraint_column_usage::dsl::constraint_column_usage
            .filter(constraint_column_usage::dsl::constraint_name.eq(constraint_name))
            .filter(constraint_column_usage::dsl::constraint_schema.eq(constraint_schema))
            .filter(constraint_column_usage::dsl::constraint_catalog.eq(constraint_catalog))
            .load::<ConstraintColumnUsage>(conn)
            .expect("Error loading constraint column usages")
    }
}

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::check_constraints)]
pub struct CheckConstraint {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub check_clause: String,
}

impl CheckConstraint {
    pub fn load_all_check_constraints(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::check_constraints::dsl::*;
        check_constraints
            .load::<CheckConstraint>(conn)
            .expect("Error loading check constraints")
    }

    pub fn load_check_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::check_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        check_constraints::dsl::check_constraints
            .filter(check_constraints::dsl::constraint_name.eq(constraint_name))
            .filter(check_constraints::dsl::constraint_schema.eq(constraint_schema))
            .filter(check_constraints::dsl::constraint_catalog.eq(constraint_catalog))
            .load::<CheckConstraint>(conn)
            .expect("Error loading check constraints")
    }
}

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::domain_constraints)]
pub struct DomainConstraint {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub domain_catalog: Option<String>,
    pub domain_schema: Option<String>,
    pub domain_name: Option<String>,
    pub is_deferrable: String,
    pub initially_deferred: String,
}

impl DomainConstraint {
    pub fn load_all_domain_constraints(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::domain_constraints::dsl::*;
        domain_constraints
            .load::<DomainConstraint>(conn)
            .expect("Error loading domain constraints")
    }

    pub fn load_domain_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::domain_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        domain_constraints::dsl::domain_constraints
            .filter(domain_constraints::dsl::constraint_name.eq(constraint_name))
            .filter(domain_constraints::dsl::constraint_schema.eq(constraint_schema))
            .filter(domain_constraints::dsl::constraint_catalog.eq(constraint_catalog))
            .load::<DomainConstraint>(conn)
            .expect("Error loading domain constraints")
    }
}
