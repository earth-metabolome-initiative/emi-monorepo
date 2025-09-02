#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl, upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(volumetric_container_model.ne(excluded(volumetric_container_model)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl, upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(volumetric_container_model.ne(excluded(volumetric_container_model)))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
