#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::icons::Icon,
> for crate::codegen::structs_codegen::tables::document_formats::DocumentFormat {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::icons::Icon,
        diesel::result::Error,
    > {
        self.icon(conn).await
    }
}
