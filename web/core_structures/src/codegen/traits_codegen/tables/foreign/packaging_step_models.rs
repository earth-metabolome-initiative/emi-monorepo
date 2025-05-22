#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    > for crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        diesel::result::Error,
    > {
        self.packaging_model(conn).await
    }
}
