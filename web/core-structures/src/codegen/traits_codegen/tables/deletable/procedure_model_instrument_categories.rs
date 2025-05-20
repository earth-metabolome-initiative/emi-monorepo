#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Deletable
for crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory {
    type Conn = diesel_async::AsyncPgConnection;
    type UserId = i32;
    async fn delete(
        &self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> Result<bool, web_common_traits::database::DeleteError> {
        use diesel_async::RunQueryDsl;
        use diesel::{QueryDsl, Identifiable};
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        if !self.can_update(user_id, conn).await? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        Ok(
            diesel::delete(Self::table().find(<&Self as Identifiable>::id(self)))
                .execute(conn)
                .await
                .map(|x| x > 0)?,
        )
    }
}
