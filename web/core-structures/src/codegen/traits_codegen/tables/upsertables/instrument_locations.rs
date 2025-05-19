#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                instrument_id
                    .ne(excluded(instrument_id))
                    .or(room_id.ne(excluded(room_id)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(created_by.ne(excluded(created_by))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                instrument_id
                    .ne(excluded(instrument_id))
                    .or(room_id.ne(excluded(room_id)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(created_by.ne(excluded(created_by))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
