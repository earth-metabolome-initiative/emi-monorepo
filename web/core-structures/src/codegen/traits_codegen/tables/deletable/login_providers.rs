#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Deletable
    for crate::codegen::structs_codegen::tables::login_providers::LoginProvider
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn delete(&self, conn: &mut Self::Conn) -> Result<usize, diesel::result::Error> {
        use diesel::{associations::HasTable, Identifiable, QueryDsl};
        use diesel_async::RunQueryDsl;
        diesel::delete(Self::table().find(self.id())).execute(conn).await
    }
}
