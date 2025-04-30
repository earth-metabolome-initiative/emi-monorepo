#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::organizations::Organization
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::organizations::organizations::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::organizations::organizations::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::organizations::organizations::name
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::organizations::organizations::name,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::organizations::organizations::url
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::organizations::organizations::url,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::organizations::organizations::country
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::organizations::organizations::country,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::organizations::organizations::alpha_two_code
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::organizations::organizations::alpha_two_code,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::organizations::organizations::state_province
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::organizations::organizations::state_province,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::organizations::organizations::domain
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::organizations::organizations::domain,
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
    for crate::codegen::structs_codegen::tables::organizations::Organization
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::organizations::organizations::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::organizations::organizations::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::organizations::organizations::name
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::organizations::organizations::name,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::organizations::organizations::url
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::organizations::organizations::url,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::organizations::organizations::country
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::organizations::organizations::country,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::organizations::organizations::alpha_two_code
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::organizations::organizations::alpha_two_code,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::organizations::organizations::state_province
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::organizations::organizations::state_province,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::organizations::organizations::domain
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::organizations::organizations::domain,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
