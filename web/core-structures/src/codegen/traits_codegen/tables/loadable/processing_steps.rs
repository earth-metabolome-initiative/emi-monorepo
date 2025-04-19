#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = rosetta_uuid::Uuid;
    async fn load(
        id: &rosetta_uuid::Uuid,
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep::table()
            .find(id)
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep::table()
            .load::<Self>(conn)
            .await
    }
}
