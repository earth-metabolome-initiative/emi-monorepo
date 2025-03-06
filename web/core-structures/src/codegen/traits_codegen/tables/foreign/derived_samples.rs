#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::units::Unit>
    for crate::codegen::structs_codegen::tables::derived_samples::DerivedSample
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::units::Unit, diesel::result::Error> {
        self.unit(conn).await
    }
}
