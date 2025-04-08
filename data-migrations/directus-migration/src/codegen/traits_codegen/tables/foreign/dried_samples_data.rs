#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::containers::Container,
    > for crate::codegen::structs_codegen::tables::dried_samples_data::DriedSamplesDatum
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    {
        self.sample_container(conn).await
    }
}
