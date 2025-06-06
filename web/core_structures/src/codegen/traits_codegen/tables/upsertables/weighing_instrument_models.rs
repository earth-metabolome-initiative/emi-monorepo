#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::weighing_instrument_models::weighing_instrument_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                error_kilograms
                    .ne(excluded(error_kilograms))
                    .or(minimum_measurable_kilograms.ne(excluded(minimum_measurable_kilograms)))
                    .or(maximum_measurable_kilograms.ne(excluded(maximum_measurable_kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::weighing_instrument_models::weighing_instrument_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                error_kilograms
                    .ne(excluded(error_kilograms))
                    .or(minimum_measurable_kilograms.ne(excluded(minimum_measurable_kilograms)))
                    .or(maximum_measurable_kilograms.ne(excluded(maximum_measurable_kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
