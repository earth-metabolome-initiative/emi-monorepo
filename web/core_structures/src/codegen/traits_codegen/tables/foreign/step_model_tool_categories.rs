#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::step_models::StepModel,
    >
    for crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory
{
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
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
>
for crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
        diesel::result::Error,
    > {
        self.procedure_model_tool_category(conn).await
    }
}
