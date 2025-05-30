#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    > for crate::codegen::structs_codegen::tables::organisms::Organism
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::trackables::Trackable, diesel::result::Error>
    {
        self.id(conn).await
    }
}
