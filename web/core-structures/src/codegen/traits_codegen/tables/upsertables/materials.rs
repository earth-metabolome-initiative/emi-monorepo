#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::materials::Material
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::materials::materials::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::materials::materials::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::materials::materials::name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::materials::materials::name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::materials::materials::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::materials::materials::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::materials::materials::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::materials::materials::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::materials::materials::color_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::materials::materials::color_id,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::materials::Material
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::materials::materials::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::materials::materials::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::materials::materials::name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::materials::materials::name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::materials::materials::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::materials::materials::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::materials::materials::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::materials::materials::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::materials::materials::color_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::materials::materials::color_id,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
