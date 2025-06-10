#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::containers::Container
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::containers::containers::*;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(container_model_id.ne(excluded(container_model_id)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::containers::Container
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::containers::containers::*;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(container_model_id.ne(excluded(container_model_id)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
