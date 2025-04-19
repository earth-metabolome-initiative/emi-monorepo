#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableTrackable
{
    type Row = crate::codegen::structs_codegen::tables::trackables::Trackable;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder;
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
        if !self.project(conn).await?.can_update(user_id, conn).await? {
            return Err(backend_request_errors::BackendRequestError::Unauthorized.into());
        }
        Ok(diesel::insert_into(Self::Row::table()).values(self).get_result(conn).await?)
    }
}
