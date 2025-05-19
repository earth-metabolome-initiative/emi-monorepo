#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
    > for crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        diesel::result::Error,
    > {
        self.id(conn).await
    }
}
