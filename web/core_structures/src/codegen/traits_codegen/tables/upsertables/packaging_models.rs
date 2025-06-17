#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::packaging_models::PackagingModel
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl, upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(trackable_id)
            .do_update()
            .set(self)
            .filter(material_id.ne(excluded(material_id)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::packaging_models::PackagingModel
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl, upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(trackable_id)
            .do_update()
            .set(self)
            .filter(material_id.ne(excluded(material_id)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
