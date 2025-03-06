#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: user_emails :: user_emails))]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub created_by: i32,
    pub login_provider_id: i16,
    pub created_at: chrono::NaiveDateTime,
    pub primary_email: bool,
}
impl UserEmail {
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .find(&self.created_by)
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn login_provider(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider::table()
            .find(&self.login_provider_id)
            .first::<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_email_and_login_provider_id(
        email: &str,
        login_provider_id: &i16,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self :: table () . filter (diesel :: BoolExpressionMethods :: and (diesel :: ExpressionMethods :: eq (crate :: codegen :: diesel_codegen :: tables :: user_emails :: user_emails :: email , email) , diesel :: ExpressionMethods :: eq (crate :: codegen :: diesel_codegen :: tables :: user_emails :: user_emails :: login_provider_id , login_provider_id))) . first :: < Self > (conn) . await . optional ()
    }
}
