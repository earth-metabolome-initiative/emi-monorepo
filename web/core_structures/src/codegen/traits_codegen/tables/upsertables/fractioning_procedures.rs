#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure
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
        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                source_processable_id
                    .ne(excluded(source_processable_id))
                    .or(destination_processable_id.ne(excluded(destination_processable_id)))
                    .or(instrument_id.ne(excluded(instrument_id)))
                    .or(kilograms.ne(excluded(kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure
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
        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                source_processable_id
                    .ne(excluded(source_processable_id))
                    .or(destination_processable_id.ne(excluded(destination_processable_id)))
                    .or(instrument_id.ne(excluded(instrument_id)))
                    .or(kilograms.ne(excluded(kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
