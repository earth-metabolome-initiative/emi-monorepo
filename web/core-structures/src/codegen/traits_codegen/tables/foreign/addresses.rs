#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::countries::Country>
    for crate::codegen::structs_codegen::tables::addresses::Address
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::countries::Country, diesel::result::Error>
    {
        self.iso(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::cities::City>
    for crate::codegen::structs_codegen::tables::addresses::Address
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::cities::City, diesel::result::Error> {
        self.city_code(conn).await
    }
}
