#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::projects::Project>
    for crate::codegen::structs_codegen::tables::nameplates::Nameplate
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::projects::Project, diesel::result::Error>
    {
        self.project(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
    > for crate::codegen::structs_codegen::tables::nameplates::Nameplate
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        diesel::result::Error,
    > {
        self.category(conn).await
    }
}
