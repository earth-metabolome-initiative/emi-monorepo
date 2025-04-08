#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
    > for crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity,
        diesel::result::Error,
    > {
        self.activity(conn).await
    }
}
