#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Deletable for crate::BallMillMachineModel {
    type Conn = diesel::PgConnection;
    type UserId = i32;
    fn delete(
        &self,
        user_id: Self::UserId,
        conn: &mut Self::Conn,
    ) -> Result<bool, web_common_traits::database::DeleteError> {
        use diesel::{Identifiable, QueryDsl, RunQueryDsl, associations::HasTable};
        use web_common_traits::database::Updatable;
        if !self.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized.into()
            );
        }
        Ok(diesel::delete(Self::table().find(<&Self as Identifiable>::id(self)))
            .execute(conn)
            .map(|x| x > 0)?)
    }
}
