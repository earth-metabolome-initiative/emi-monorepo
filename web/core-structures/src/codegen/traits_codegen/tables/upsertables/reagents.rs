#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::reagents::Reagent
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::reagents::reagents::table,
            )
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::reagents::reagents::id)
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
                                            crate::codegen::diesel_codegen::tables::reagents::reagents::name
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::reagents::reagents::name,
                                                    ),
                                                ),
                                            crate::codegen::diesel_codegen::tables::reagents::reagents::description
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::reagents::reagents::description,
                                                    ),
                                                ),
                                        ),
                                        crate::codegen::diesel_codegen::tables::reagents::reagents::purity
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::reagents::reagents::purity,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::reagents::reagents::gram_per_mole
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::reagents::reagents::gram_per_mole,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::reagents::reagents::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::reagents::reagents::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::reagents::reagents::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::reagents::reagents::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::reagents::reagents::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::reagents::reagents::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::reagents::reagents::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::reagents::reagents::updated_at,
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
    for crate::codegen::structs_codegen::tables::reagents::Reagent
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::reagents::reagents::table,
            )
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::reagents::reagents::id)
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
                                            crate::codegen::diesel_codegen::tables::reagents::reagents::name
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::reagents::reagents::name,
                                                    ),
                                                ),
                                            crate::codegen::diesel_codegen::tables::reagents::reagents::description
                                                .ne(
                                                    diesel::upsert::excluded(
                                                        crate::codegen::diesel_codegen::tables::reagents::reagents::description,
                                                    ),
                                                ),
                                        ),
                                        crate::codegen::diesel_codegen::tables::reagents::reagents::purity
                                            .ne(
                                                diesel::upsert::excluded(
                                                    crate::codegen::diesel_codegen::tables::reagents::reagents::purity,
                                                ),
                                            ),
                                    ),
                                    crate::codegen::diesel_codegen::tables::reagents::reagents::gram_per_mole
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::reagents::reagents::gram_per_mole,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::reagents::reagents::created_by
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::reagents::reagents::created_by,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::reagents::reagents::created_at
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::reagents::reagents::created_at,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::reagents::reagents::updated_by
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::reagents::reagents::updated_by,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::reagents::reagents::updated_at
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::reagents::reagents::updated_at,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
