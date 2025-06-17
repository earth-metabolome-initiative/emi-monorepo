#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::centrifugable_container_models::centrifugable_container_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((centrifuged_with, container_model_id))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::centrifugable_container_models::centrifugable_container_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((centrifuged_with, container_model_id))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
