#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::batches::Batch>
    for crate::codegen::structs_codegen::tables::projects::Project
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::batches::Batch, diesel::result::Error>
    {
        self.batch(conn).await
    }
}
