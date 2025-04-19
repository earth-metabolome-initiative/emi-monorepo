#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::addresses::Address>
    for crate::codegen::structs_codegen::tables::rooms::Room
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::addresses::Address, diesel::result::Error>
    {
        self.addresses(conn).await
    }
}
