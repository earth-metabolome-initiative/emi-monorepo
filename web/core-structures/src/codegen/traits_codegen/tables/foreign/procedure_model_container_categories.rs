#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
>
for crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory {
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
    crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
>
for crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
        diesel::result::Error,
    > {
        self.container_category(conn).await
    }
}
