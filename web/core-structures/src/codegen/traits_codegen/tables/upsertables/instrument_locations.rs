#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::instrument_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::instrument_id,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::room_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::room_id,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_by
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_by,
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
    for crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::instrument_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::instrument_id,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::room_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::room_id,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_by
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::created_by,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
