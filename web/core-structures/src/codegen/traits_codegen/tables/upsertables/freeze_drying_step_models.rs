#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    diesel::BoolExpressionMethods::and(
                                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_kelvin
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_kelvin,
                                                ),
                                            ),
                                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_pascal
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_pascal,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_seconds
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_seconds,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_at,
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
    for crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    diesel::BoolExpressionMethods::and(
                                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_kelvin
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_kelvin,
                                                ),
                                            ),
                                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_pascal
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_pascal,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_seconds
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::expected_seconds,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models::updated_at,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
