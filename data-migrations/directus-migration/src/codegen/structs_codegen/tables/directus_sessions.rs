#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(token))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions
)]
pub struct DirectusSession {
    pub token: String,
    pub user: Option<uuid::Uuid>,
    pub expires: chrono::NaiveDateTime,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub share: Option<uuid::Uuid>,
    pub origin: Option<String>,
    pub next_token: Option<String>,
}
impl DirectusSession {
    #[cfg(feature = "postgres")]
    pub async fn user(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(user) = self.user.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .find(user)
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn share(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_shares::DirectusShare>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(share) = self.share.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_shares::DirectusShare::table()
            .find(share)
            .first::<
                crate::codegen::structs_codegen::tables::directus_shares::DirectusShare,
            >(conn)
            .await
            .map(Some)
    }
}
