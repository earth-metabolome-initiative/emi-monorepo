#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(token))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions
)]
pub struct DirectusSession {
    pub token: String,
    pub user: Option<rosetta_uuid::Uuid>,
    pub expires: rosetta_timestamp::TimestampUTC,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub share: Option<rosetta_uuid::Uuid>,
    pub origin: Option<String>,
    pub next_token: Option<String>,
}
impl diesel::Identifiable for DirectusSession {
    type Id = String;
    fn id(self) -> Self::Id {
        self.token
    }
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(user) = self.user.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>(conn)
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(share) = self.share.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_shares::DirectusShare::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_shares::directus_shares::dsl::id
                    .eq(share),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_shares::DirectusShare>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user(
        conn: &mut diesel_async::AsyncPgConnection,
        user: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions::dsl::user
                    .eq(user.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_share(
        conn: &mut diesel_async::AsyncPgConnection,
        share: &crate::codegen::structs_codegen::tables::directus_shares::DirectusShare,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions::dsl::share
                    .eq(share.id),
            )
            .load::<Self>(conn)
            .await
    }
}
