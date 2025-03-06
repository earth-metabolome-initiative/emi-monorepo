#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::samples::Sample>
    for crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::samples::Sample, diesel::result::Error>
    {
        self.sample(conn).await
    }
}
