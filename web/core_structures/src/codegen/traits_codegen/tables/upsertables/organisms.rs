#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::organisms::Organism
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::organisms::organisms::*;
        use diesel::RunQueryDsl;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_nothing()
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::organisms::Organism
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::organisms::organisms::*;
        use diesel::RunQueryDsl;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_nothing()
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
