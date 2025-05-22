#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
    > for crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
        diesel::result::Error,
    > {
        self.flow(conn).await
    }
}
