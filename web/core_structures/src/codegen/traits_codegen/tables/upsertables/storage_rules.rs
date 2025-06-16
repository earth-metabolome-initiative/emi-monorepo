#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::storage_rules::StorageRule
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl, upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::storage_rules::storage_rules::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((parent_container_id, child_container_id))
            .do_update()
            .set(self)
            .filter(quantity.ne(excluded(quantity)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::storage_rules::StorageRule
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl, upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::storage_rules::storage_rules::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((parent_container_id, child_container_id))
            .do_update()
            .set(self)
            .filter(quantity.ne(excluded(quantity)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
