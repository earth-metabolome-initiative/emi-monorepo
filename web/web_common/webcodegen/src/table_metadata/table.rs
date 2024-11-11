use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};
use itertools::Itertools;

use crate::Column;
use crate::TableConstraint;

/// Struct defining the `information_schema.tables` table.
#[derive(Queryable, QueryableByName, PartialEq, Eq, Selectable, Debug)]
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

    pub fn column_by_name(
        &self,
        conn: &mut PgConnection,
        column_name: &str,
    ) -> Result<Column, DieselError> {
        use crate::schema::columns;
        columns::dsl::columns
            .filter(columns::dsl::table_name.eq(&self.table_name))
            .filter(columns::dsl::table_schema.eq(&self.table_schema))
            .filter(columns::dsl::table_catalog.eq(&self.table_catalog))
            .filter(columns::dsl::column_name.eq(column_name))
            .first::<Column>(conn)
    }

    pub fn unique_columns(&self, conn: &mut PgConnection) -> Result<Vec<Vec<Column>>, DieselError> {
        use crate::schema::columns;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        key_column_usage::dsl::key_column_usage
            .inner_join(
                columns::dsl::columns.on(key_column_usage::dsl::table_name
                    .nullable()
                    .eq(columns::dsl::table_name.nullable())
                    .and(
                        key_column_usage::dsl::table_schema
                            .nullable()
                            .eq(columns::dsl::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::table_catalog
                            .nullable()
                            .eq(columns::dsl::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::column_name
                            .nullable()
                            .eq(columns::dsl::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::dsl::table_constraints.on(
                    key_column_usage::dsl::constraint_name
                        .nullable()
                        .eq(table_constraints::dsl::constraint_name.nullable())
                        .and(
                            key_column_usage::dsl::constraint_schema
                                .nullable()
                                .eq(table_constraints::dsl::constraint_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::constraint_catalog
                                .nullable()
                                .eq(table_constraints::dsl::constraint_catalog.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_name
                                .nullable()
                                .eq(table_constraints::dsl::table_name.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_schema
                                .nullable()
                                .eq(table_constraints::dsl::table_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_catalog
                                .nullable()
                                .eq(table_constraints::dsl::table_catalog.nullable()),
                        ),
                ),
            )
            .filter(key_column_usage::dsl::table_name.eq(&self.table_name))
            .filter(key_column_usage::dsl::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::dsl::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::dsl::constraint_type.eq("UNIQUE"))
            .order_by(table_constraints::dsl::constraint_name)
            .select((TableConstraint::as_select(), Column::as_select()))
            .load::<(TableConstraint, Column)>(conn)
            .map(|rows| {
                rows.into_iter()
                    .chunk_by(|(constraint, _)| constraint.constraint_name.clone())
                    .into_iter()
                    .map(|(_, group)| {
                        group
                            .into_iter()
                            .map(|(_, column)| column)
                            .collect::<Vec<Column>>()
                    })
                    .collect()
            })
    }

    pub fn primary_key_columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, DieselError> {
        use crate::schema::columns;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        key_column_usage::dsl::key_column_usage
            .inner_join(
                columns::dsl::columns.on(key_column_usage::dsl::table_name
                    .nullable()
                    .eq(columns::dsl::table_name.nullable())
                    .and(
                        key_column_usage::dsl::table_schema
                            .nullable()
                            .eq(columns::dsl::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::table_catalog
                            .nullable()
                            .eq(columns::dsl::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::column_name
                            .nullable()
                            .eq(columns::dsl::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::dsl::table_constraints.on(
                    key_column_usage::dsl::constraint_name
                        .nullable()
                        .eq(table_constraints::dsl::constraint_name.nullable())
                        .and(
                            key_column_usage::dsl::constraint_schema
                                .nullable()
                                .eq(table_constraints::dsl::constraint_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::constraint_catalog
                                .nullable()
                                .eq(table_constraints::dsl::constraint_catalog.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_name
                                .nullable()
                                .eq(table_constraints::dsl::table_name.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_schema
                                .nullable()
                                .eq(table_constraints::dsl::table_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_catalog
                                .nullable()
                                .eq(table_constraints::dsl::table_catalog.nullable()),
                        ),
                ),
            )
            .filter(key_column_usage::dsl::table_name.eq(&self.table_name))
            .filter(key_column_usage::dsl::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::dsl::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::dsl::constraint_type.eq("PRIMARY KEY"))
            .select(Column::as_select())
            .load::<Column>(conn)
    }
}
