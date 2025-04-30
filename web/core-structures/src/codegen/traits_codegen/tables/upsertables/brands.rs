#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::brands::Brand
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::brands::brands::table,
            )
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::brands::brands::id)
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::brands::brands::name
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::brands::brands::name,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::brands::brands::created_by
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::brands::brands::created_by,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::brands::brands::created_at
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::brands::brands::created_at,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::brands::brands::updated_by
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::brands::brands::updated_by,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::brands::brands::updated_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::brands::brands::updated_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::brands::brands::brand_state_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::brands::brands::brand_state_id,
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
    for crate::codegen::structs_codegen::tables::brands::Brand
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::brands::brands::table,
            )
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::brands::brands::id)
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::brands::brands::name
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::brands::brands::name,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::brands::brands::created_by
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::brands::brands::created_by,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::brands::brands::created_at
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::brands::brands::created_at,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::brands::brands::updated_by
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::brands::brands::updated_by,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::brands::brands::updated_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::brands::brands::updated_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::brands::brands::brand_state_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::brands::brands::brand_state_id,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
