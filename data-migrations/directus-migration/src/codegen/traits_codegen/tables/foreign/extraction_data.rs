#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::si_units::SiUnit,
> for crate::codegen::structs_codegen::tables::extraction_data::ExtractionDatum {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::si_units::SiUnit,
        diesel::result::Error,
    > {
        self.dried_weight_unit(conn).await
    }
}
