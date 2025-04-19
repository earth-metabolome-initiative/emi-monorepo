#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
>
for crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        diesel::result::Error,
    > {
        self.procedure_model(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::tool_categories::ToolCategory,
>
for crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::tool_categories::ToolCategory,
        diesel::result::Error,
    > {
        self.tool_category(conn).await
    }
}
