#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModel
{
    type Row =
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder;
    type Conn = diesel_async::AsyncPgConnection;
    type UserId = i32;
    async fn insert(
        self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            <Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute,
        >,
    > {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        use web_common_traits::database::Updatable;
        if !self.parent_procedure_model(conn).await?.can_update(user_id, conn).await? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized.into()
            );
        }
        if !self.child_procedure_model(conn).await?.can_update(user_id, conn).await? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized.into()
            );
        }
        Ok(diesel::insert_into(Self::Row::table()).values(self).get_result(conn).await?)
    }
}
