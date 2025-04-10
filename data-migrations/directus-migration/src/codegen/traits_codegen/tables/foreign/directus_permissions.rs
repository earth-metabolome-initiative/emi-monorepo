#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
> for crate::codegen::structs_codegen::tables::directus_permissions::DirectusPermission {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
        diesel::result::Error,
    > {
        self.policy(conn).await
    }
}
