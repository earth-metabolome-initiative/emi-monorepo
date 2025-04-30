#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::id,
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
                                        diesel::BoolExpressionMethods::and(
                                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::step_model_id
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::step_model_id,
                                                    ),
                                                ),
                                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::container_category_id
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::container_category_id,
                                                    ),
                                                ),
                                        ),
                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::expected_kelvin
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::expected_kelvin,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::tolerance_kelvin
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::tolerance_kelvin,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_at,
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
for crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::id,
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
                                        diesel::BoolExpressionMethods::and(
                                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::step_model_id
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::step_model_id,
                                                    ),
                                                ),
                                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::container_category_id
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::container_category_id,
                                                    ),
                                                ),
                                        ),
                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::expected_kelvin
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::expected_kelvin,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::tolerance_kelvin
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::tolerance_kelvin,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::updated_at,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
