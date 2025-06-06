#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable
)]
#[diesel(primary_key(token))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions
)]
pub struct DirectusSession {
    pub token: String,
    pub user: Option<::rosetta_uuid::Uuid>,
    pub expires: ::rosetta_timestamp::TimestampUTC,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub share: Option<::rosetta_uuid::Uuid>,
    pub origin: Option<String>,
    pub next_token: Option<String>,
}
impl web_common_traits::prelude::TableName for DirectusSession {
    const TABLE_NAME: &'static str = "directus_sessions";
}
impl diesel::Identifiable for DirectusSession {
    type Id = String;
    fn id(self) -> Self::Id {
        self.token
    }
}
impl DirectusSession {
    pub fn share<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_shares::DirectusShare>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_shares::DirectusShare: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_shares::DirectusShare as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_shares::DirectusShare as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_shares::DirectusShare as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_shares::DirectusShare as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_shares::DirectusShare as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_shares::DirectusShare as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_shares::DirectusShare,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{RunQueryDsl, QueryDsl};
        let Some(share) = self.share else {
            return Ok(None);
        };
        RunQueryDsl::first(
                QueryDsl::find(
                    crate::codegen::structs_codegen::tables::directus_shares::DirectusShare::table(),
                    share,
                ),
                conn,
            )
            .map(Some)
    }
    pub fn user<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{RunQueryDsl, QueryDsl};
        let Some(user) = self.user else {
            return Ok(None);
        };
        RunQueryDsl::first(
                QueryDsl::find(
                    crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table(),
                    user,
                ),
                conn,
            )
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user(
        user: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::user.eq(user))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_expires(
        expires: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::expires.eq(expires))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_ip(
        ip: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::ip.eq(ip))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_agent(
        user_agent: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::user_agent.eq(user_agent))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_share(
        share: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::share.eq(share))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_origin(
        origin: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::origin.eq(origin))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_next_token(
        next_token: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::next_token.eq(next_token))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusSession> for DirectusSession {
    fn as_ref(&self) -> &DirectusSession {
        self
    }
}
