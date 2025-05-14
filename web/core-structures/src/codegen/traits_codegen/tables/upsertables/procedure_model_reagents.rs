#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::id,
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
                                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::procedure_model_id
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::procedure_model_id,
                                                ),
                                            ),
                                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::reagent_id
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::reagent_id,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::quantity
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::quantity,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_at,
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
    for crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::id,
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
                                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::procedure_model_id
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::procedure_model_id,
                                                ),
                                            ),
                                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::reagent_id
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::reagent_id,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::quantity
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::quantity,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::procedure_model_reagents::procedure_model_reagents::updated_at,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
