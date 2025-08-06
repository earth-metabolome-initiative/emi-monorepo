#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::fractioning_procedure_models::fractioning_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                kilograms
                    .ne(excluded(kilograms))
                    .or(tolerance_percentage.ne(excluded(tolerance_percentage)))
                    .or(weighed_with.ne(excluded(weighed_with)))
                    .or(procedure_weighed_with.ne(excluded(procedure_weighed_with)))
                    .or(
                        procedure_fragment_source.ne(excluded(procedure_fragment_source)),
                    )
                    .or(fragment_placed_into.ne(excluded(fragment_placed_into)))
                    .or(
                        procedure_fragment_placed_into
                            .ne(excluded(procedure_fragment_placed_into)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::fractioning_procedure_models::fractioning_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                kilograms
                    .ne(excluded(kilograms))
                    .or(tolerance_percentage.ne(excluded(tolerance_percentage)))
                    .or(weighed_with.ne(excluded(weighed_with)))
                    .or(procedure_weighed_with.ne(excluded(procedure_weighed_with)))
                    .or(
                        procedure_fragment_source.ne(excluded(procedure_fragment_source)),
                    )
                    .or(fragment_placed_into.ne(excluded(fragment_placed_into)))
                    .or(
                        procedure_fragment_placed_into
                            .ne(excluded(procedure_fragment_placed_into)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
