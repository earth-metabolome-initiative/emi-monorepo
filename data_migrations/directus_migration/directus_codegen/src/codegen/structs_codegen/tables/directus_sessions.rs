#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_shares::DirectusShare,
        foreign_key = share
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        foreign_key = user
    )
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
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_sessions::DirectusSession,
    > for DirectusSession
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a String>,
{
}
impl diesel::Identifiable for DirectusSession {
    type Id = String;
    fn id(self) -> Self::Id {
        self.token.clone()
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusSession {
    type PrimaryKey = String;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.token.clone()
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
        crate::codegen::structs_codegen::tables::directus_shares::DirectusShare:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(share) = self.share else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_shares::DirectusShare::read(share, conn)
            .optional()
    }
    pub fn user<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(user) = self.user else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(user, conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_ip(
        ip: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

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
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_sessions::directus_sessions;
        Self::table()
            .filter(directus_sessions::user_agent.eq(user_agent))
            .order_by(directus_sessions::token.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_origin(
        origin: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

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
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

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
