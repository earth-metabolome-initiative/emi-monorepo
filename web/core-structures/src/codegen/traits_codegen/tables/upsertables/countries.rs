#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::countries::Country
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(crate::codegen::diesel_codegen::tables::countries::countries::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::countries::countries::iso)
            .do_update()
            .set(self)
            .filter(crate::codegen::diesel_codegen::tables::countries::countries::name.ne(
                diesel::upsert::excluded(
                    crate::codegen::diesel_codegen::tables::countries::countries::name,
                ),
            ))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::countries::Country
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(crate::codegen::diesel_codegen::tables::countries::countries::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::countries::countries::iso)
            .do_update()
            .set(self)
            .filter(crate::codegen::diesel_codegen::tables::countries::countries::name.ne(
                diesel::upsert::excluded(
                    crate::codegen::diesel_codegen::tables::countries::countries::name,
                ),
            ))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
