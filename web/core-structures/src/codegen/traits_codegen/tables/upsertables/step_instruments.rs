#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::step_instruments::StepInstrument
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::step_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::step_id,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::instrument_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::instrument_id,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_at,
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
    for crate::codegen::structs_codegen::tables::step_instruments::StepInstrument
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::step_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::step_id,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::instrument_id
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::instrument_id,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::step_instruments::step_instruments::created_at,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
