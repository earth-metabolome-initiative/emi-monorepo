#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
>
for crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    > {
        self.id(conn).await
    }
}
