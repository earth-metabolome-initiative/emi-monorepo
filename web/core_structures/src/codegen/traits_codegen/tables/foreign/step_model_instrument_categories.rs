#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::step_models::StepModel,
>
for crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_models::StepModel,
        diesel::result::Error,
    > {
        self.step_model(conn).await
    }
}
