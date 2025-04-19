#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
for crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory {
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = i32;
    async fn load(
        id: &i32,
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory::table()
            .find(id)
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(
        conn: &mut Self::Conn,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory::table()
            .load::<Self>(conn)
            .await
    }
}
