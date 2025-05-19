#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    > for crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent
{
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
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::reagents::Reagent>
    for crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::reagents::Reagent, diesel::result::Error>
    {
        self.reagent(conn).await
    }
}
