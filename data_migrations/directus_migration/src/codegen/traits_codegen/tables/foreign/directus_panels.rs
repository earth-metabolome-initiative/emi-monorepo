#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard,
    > for crate::codegen::structs_codegen::tables::directus_panels::DirectusPanel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard,
        diesel::result::Error,
    > {
        self.dashboard(conn).await
    }
}
