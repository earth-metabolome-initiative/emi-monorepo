#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::color_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::color_id,
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
    for crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::color_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects::color_id,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
