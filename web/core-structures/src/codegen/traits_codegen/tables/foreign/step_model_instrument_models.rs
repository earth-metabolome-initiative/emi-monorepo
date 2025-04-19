#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
>
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        diesel::result::Error,
    > {
        self.id(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
>
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    > {
        self.instrument_model(conn).await
    }
}
