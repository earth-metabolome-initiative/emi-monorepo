#[derive(Debug, Clone, PartialEq)]
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
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
        foreign_key = role
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_users::directus_users
)]
pub struct DirectusUser {
    pub id: ::rosetta_uuid::Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<::serde_json::Value>,
    pub avatar: Option<::rosetta_uuid::Uuid>,
    pub language: Option<String>,
    pub tfa_secret: Option<String>,
    pub status: String,
    pub role: Option<::rosetta_uuid::Uuid>,
    pub token: Option<String>,
    pub last_access: Option<::rosetta_timestamp::TimestampUTC>,
    pub last_page: Option<String>,
    pub provider: String,
    pub external_identifier: Option<String>,
    pub auth_data: Option<::serde_json::Value>,
    pub email_notifications: Option<bool>,
    pub appearance: Option<String>,
    pub theme_dark: Option<String>,
    pub theme_light: Option<String>,
    pub theme_light_overrides: Option<::serde_json::Value>,
    pub theme_dark_overrides: Option<::serde_json::Value>,
}
impl web_common_traits::prelude::TableName for DirectusUser {
    const TABLE_NAME: &'static str = "directus_users";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    > for DirectusUser
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for DirectusUser {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusUser {
    pub fn role<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(role) = self.role else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::read(role, conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_email(
        email: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::email.eq(email))
            .order_by(directus_users::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_external_identifier(
        external_identifier: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::external_identifier.eq(external_identifier))
            .order_by(directus_users::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_token(
        token: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::token.eq(token))
            .order_by(directus_users::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_first_name(
        first_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::first_name.eq(first_name))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_last_name(
        last_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::last_name.eq(last_name))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_password(
        password: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::password.eq(password))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_location(
        location: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::location.eq(location))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_title(
        title: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::title.eq(title))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::description.eq(description))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_avatar(
        avatar: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::avatar.eq(avatar))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_language(
        language: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::language.eq(language))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_tfa_secret(
        tfa_secret: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::tfa_secret.eq(tfa_secret))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::status.eq(status))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_last_access(
        last_access: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::last_access.eq(last_access))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_last_page(
        last_page: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::last_page.eq(last_page))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_provider(
        provider: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::provider.eq(provider))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_email_notifications(
        email_notifications: bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::email_notifications.eq(email_notifications))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_appearance(
        appearance: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::appearance.eq(appearance))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_theme_dark(
        theme_dark: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::theme_dark.eq(theme_dark))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_theme_light(
        theme_light: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
        Self::table()
            .filter(directus_users::theme_light.eq(theme_light))
            .order_by(directus_users::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusUser> for DirectusUser {
    fn as_ref(&self) -> &DirectusUser {
        self
    }
}
