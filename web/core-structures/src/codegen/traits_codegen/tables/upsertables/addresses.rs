#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::addresses::Address
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::addresses::addresses::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::addresses::addresses::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                crate::codegen::diesel_codegen::tables::addresses::addresses::city_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::addresses::addresses::city_id,
                                        ),
                                    ),
                                crate::codegen::diesel_codegen::tables::addresses::addresses::street_name
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::addresses::addresses::street_name,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::addresses::addresses::street_number
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::addresses::addresses::street_number,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::addresses::addresses::postal_code
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::addresses::addresses::postal_code,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::addresses::addresses::geolocation
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::addresses::addresses::geolocation,
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
    for crate::codegen::structs_codegen::tables::addresses::Address
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::addresses::addresses::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::addresses::addresses::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                crate::codegen::diesel_codegen::tables::addresses::addresses::city_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::addresses::addresses::city_id,
                                        ),
                                    ),
                                crate::codegen::diesel_codegen::tables::addresses::addresses::street_name
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::addresses::addresses::street_name,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::addresses::addresses::street_number
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::addresses::addresses::street_number,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::addresses::addresses::postal_code
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::addresses::addresses::postal_code,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::addresses::addresses::geolocation
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::addresses::addresses::geolocation,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
