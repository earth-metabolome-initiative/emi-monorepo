#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::instrument_model_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::instrument_model_id,
                                        ),
                                    ),
                                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_at,
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
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::instrument_model_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::instrument_model_id,
                                        ),
                                    ),
                                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::updated_at,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
