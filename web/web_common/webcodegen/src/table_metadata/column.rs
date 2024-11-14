use crate::Table;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, Queryable, QueryableByName,
    RunQueryDsl, Selectable, SelectableHelper,
};
use syn::Type;

use crate::table_metadata::sql_function::postgres_type_to_diesel;
use crate::KeyColumnUsage;

/// Struct defining the `information_schema.columns` table.
#[derive(Queryable, QueryableByName, Selectable, PartialEq, Eq, Debug)]
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
    /// Returns the Syn rust type of the column.
    pub fn rust_type(&self) -> Type {
        use syn::parse_str;
        let rust_type = match self.data_type.as_str() {
            "integer" => "i32",
            "character varying" => "String",
            "text" => "String",
            "timestamp with time zone" => "chrono::NaiveDateTime",
            "timestamp without time zone" => "chrono::NaiveDateTime",
            "date" => "chrono::NaiveDate",
            "boolean" => "bool",
            "numeric" => "f64",
            "uuid" => "uuid::Uuid",
            "jsonb" => "serde_json::Value",
            "json" => "serde_json::Value",
            "bytea" => "Vec<u8>",
            "inet" => "std::net::IpAddr",
            "time without time zone" => "chrono::NaiveTime",
            "time with time zone" => "chrono::NaiveTime",
            "interval" => "chrono::Duration",
            "bit" => "Vec<u8>",
            "bit varying" => "Vec<u8>",
            "money" => "BigDecimal",
            "xml" => "String",
            "oid" => "u32",
            "smallint" => "i16",
            "bigint" => "i64",
            "real" => "f32",
            "double precision" => "f64",
            "character" => "String",
            "char" => "String",
            "cidr" => "std::net::IpAddr",
            "macaddr" => "std::net::MacAddr",
            "macaddr8" => "std::net::MacAddr",
            "point" => "Point",
            "line" => "Line",
            "lseg" => "LineSegment",
            other => todo!("Unsupported data type: {}", other),
        };

        if self.is_nullable() {
            parse_str(&format!("Option<{}>", rust_type)).unwrap()
        } else {
            parse_str(rust_type).unwrap()
        }
    }

    pub fn is_nullable(&self) -> bool {
        self.__is_nullable == "YES"
    }

    pub fn diesel_type(&self) -> Type {
        postgres_type_to_diesel(&self.data_type, self.is_nullable())
    }

    pub fn is_uuid(&self) -> bool {
        self.data_type == "uuid"
    }

    pub fn is_automatically_generated(&self) -> bool {
        self.is_generated == "ALWAYS"
            || self
                .column_default
                .as_ref()
                .map_or(false, |d| d.starts_with("nextval"))
            || self
                .column_default
                .as_ref()
                .map_or(false, |d| d.starts_with("CURRENT_TIMESTAMP"))
            || self.is_identity.as_ref().map_or(false, |i| i == "YES")
    }

    pub fn is_updated_by(&self, conn: &mut PgConnection) -> bool {
        self.column_name == "updated_by" && self.is_session_user_generated(conn)
    }

    pub fn is_created_by(&self, conn: &mut PgConnection) -> bool {
        self.column_name == "created_by" && self.is_session_user_generated(conn)
    }

    pub fn is_session_user_generated(&self, conn: &mut PgConnection) -> bool {
        // A column is associated to the user section if:
        // - it is a foreign key of the users table
        // - it is named `updated_by` or `created_by`

        if self.column_name != "updated_by" && self.column_name != "created_by" {
            return false;
        }

        if let Ok(Some((table, _))) = self.foreign_table(conn) {
            if table.table_name == "users" {
                return true;
            }
        }

        false
    }

    pub fn load_all(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::columns::dsl::*;
        columns.load::<Column>(conn).expect("Error loading columns")
    }

    pub fn is_foreign_key(&self, conn: &mut PgConnection) -> bool {
        use crate::schema::key_column_usage;
        use crate::schema::referential_constraints;
        key_column_usage::dsl::key_column_usage
            .inner_join(
                referential_constraints::dsl::referential_constraints.on(
                    key_column_usage::dsl::constraint_name
                        .eq(referential_constraints::dsl::constraint_name)
                        .and(
                            key_column_usage::dsl::constraint_schema
                                .eq(referential_constraints::dsl::constraint_schema),
                        )
                        .and(
                            key_column_usage::dsl::constraint_catalog
                                .eq(referential_constraints::dsl::constraint_catalog),
                        ),
                ),
            )
            .filter(key_column_usage::dsl::column_name.eq(&self.column_name))
            .filter(key_column_usage::dsl::table_name.eq(&self.table_name))
            .filter(key_column_usage::dsl::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::dsl::table_catalog.eq(&self.table_catalog))
            .select(KeyColumnUsage::as_select())
            .first::<KeyColumnUsage>(conn)
            .is_ok()
    }

    ///

    /// Returns the foreign table of the column if it is a foreign key.
    /// If the column is not a foreign key, returns `None`.
    pub fn foreign_table(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<(Table, Column)>, DieselError> {
        use crate::schema::columns;
        use crate::schema::constraint_column_usage;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        use crate::schema::tables;
        table_constraints::dsl::table_constraints
            .inner_join(
                key_column_usage::dsl::key_column_usage.on(table_constraints::dsl::constraint_name
                    .eq(key_column_usage::dsl::constraint_name)
                    .and(
                        table_constraints::dsl::constraint_schema
                            .eq(key_column_usage::dsl::constraint_schema),
                    )
                    .and(
                        table_constraints::dsl::constraint_catalog
                            .eq(key_column_usage::dsl::constraint_catalog),
                    )
                    .and(table_constraints::dsl::table_name.eq(key_column_usage::dsl::table_name))
                    .and(
                        table_constraints::dsl::table_schema
                            .eq(key_column_usage::dsl::table_schema),
                    )
                    .and(
                        table_constraints::dsl::table_catalog
                            .eq(key_column_usage::dsl::table_catalog),
                    )),
            )
            .inner_join(
                constraint_column_usage::dsl::constraint_column_usage
                    .on(constraint_column_usage::dsl::constraint_name
                        .eq(table_constraints::dsl::constraint_name)),
            )
            .inner_join(
                tables::dsl::tables.on(tables::dsl::table_name
                    .eq(constraint_column_usage::dsl::table_name)
                    .and(tables::dsl::table_schema.eq(constraint_column_usage::dsl::table_schema))
                    .and(
                        tables::dsl::table_catalog.eq(constraint_column_usage::dsl::table_catalog),
                    )),
            )
            .inner_join(
                columns::dsl::columns.on(columns::dsl::table_name
                    .eq(constraint_column_usage::dsl::table_name)
                    .and(columns::dsl::table_schema.eq(constraint_column_usage::dsl::table_schema))
                    .and(
                        columns::dsl::table_catalog.eq(constraint_column_usage::dsl::table_catalog),
                    )
                    .and(columns::dsl::column_name.eq(constraint_column_usage::dsl::column_name))),
            )
            .filter(table_constraints::dsl::constraint_type.eq("FOREIGN KEY"))
            .filter(table_constraints::dsl::table_name.eq(&self.table_name))
            .filter(table_constraints::dsl::table_schema.eq(&self.table_schema))
            .filter(table_constraints::dsl::table_catalog.eq(&self.table_catalog))
            .filter(key_column_usage::dsl::column_name.eq(&self.column_name))
            .select((Table::as_select(), Column::as_select()))
            .first::<(Table, Column)>(conn)
            .map(Some)
            .or_else(|e| {
                if e == DieselError::NotFound {
                    Ok(None)
                } else {
                    Err(e)
                }
            })
    }
}
