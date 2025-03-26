#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
>
for crate::codegen::structs_codegen::tables::directus_notifications::DirectusNotification {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        diesel::result::Error,
    > {
        self.recipient(conn).await
    }
}
