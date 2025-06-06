#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::aliquoting_instrument_models::aliquoting_instrument_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                error_liters
                    .ne(excluded(error_liters))
                    .or(
                        minimum_measurable_liters.ne(excluded(minimum_measurable_liters)),
                    )
                    .or(
                        maximum_measurable_liters.ne(excluded(maximum_measurable_liters)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::aliquoting_instrument_models::aliquoting_instrument_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                error_liters
                    .ne(excluded(error_liters))
                    .or(
                        minimum_measurable_liters.ne(excluded(minimum_measurable_liters)),
                    )
                    .or(
                        maximum_measurable_liters.ne(excluded(maximum_measurable_liters)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
