#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::source_processable_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::source_processable_id,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::destination_processable_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::destination_processable_id,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::instrument_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::instrument_id,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::kilograms
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::kilograms,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_at,
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
    for crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::source_processable_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::source_processable_id,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::destination_processable_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::destination_processable_id,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::instrument_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::instrument_id,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::kilograms
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::kilograms,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps::created_at,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
