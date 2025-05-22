#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::documents::Document,
    > for crate::codegen::structs_codegen::tables::step_models::StepModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::documents::Document, diesel::result::Error>
    {
        self.photograph(conn).await
    }
}
