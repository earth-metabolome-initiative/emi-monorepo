#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::instrument_states::InstrumentState
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::color_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::color_id,
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
    for crate::codegen::structs_codegen::tables::instrument_states::InstrumentState
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::color_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::color_id,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
