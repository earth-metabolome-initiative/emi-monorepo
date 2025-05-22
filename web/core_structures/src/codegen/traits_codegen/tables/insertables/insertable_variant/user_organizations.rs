#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization
{
    async fn backend_insert(
        self,
        conn: &mut Self::Conn,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            <Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute,
        >,
    > {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        Ok(diesel::insert_into(Self::Row::table()).values(self).get_result(conn).await?)
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization
{
    type Row = crate::codegen::structs_codegen::tables::user_organizations::UserOrganization;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder;
    type Conn = diesel_async::AsyncPgConnection;
    type UserId = i32;
    async fn insert(
        self,
        _user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            <Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute,
        >,
    > {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        Ok(diesel::insert_into(Self::Row::table()).values(self).get_result(conn).await?)
    }
}
