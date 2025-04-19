#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::photographs::Photograph,
    > for crate::codegen::structs_codegen::tables::step_models::StepModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photographs::Photograph,
        diesel::result::Error,
    > {
        self.photograph(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
    > for crate::codegen::structs_codegen::tables::step_models::StepModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
        diesel::result::Error,
    > {
        self.step_model_category(conn).await
    }
}
