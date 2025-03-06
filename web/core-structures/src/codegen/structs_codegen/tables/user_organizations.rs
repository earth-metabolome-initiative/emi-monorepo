#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(user_id, organization_id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: user_organizations :: user_organizations))]
pub struct UserOrganization {
    pub user_id: i32,
    pub organization_id: i16,
}
impl UserOrganization {
    #[cfg(feature = "postgres")]
    pub async fn user(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .find(&self.user_id)
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn organization(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organizations::Organization,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::organizations::Organization::table()
            .find(&self.organization_id)
            .first::<crate::codegen::structs_codegen::tables::organizations::Organization>(conn)
            .await
    }
}
