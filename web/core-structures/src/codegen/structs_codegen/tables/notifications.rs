#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: notifications :: notifications))]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub record: String,
    pub read: bool,
}
impl Notification {
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
}
