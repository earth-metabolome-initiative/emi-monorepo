#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "32-column-tables",
    derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)
)]
#[cfg_attr(feature = "32-column-tables", diesel(primary_key(id)))]
#[cfg_attr(
    feature = "32-column-tables",
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::directus_users::directus_users
    )
)]
pub struct DirectusUser {
    pub id: uuid::Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub avatar: Option<uuid::Uuid>,
    pub language: Option<String>,
    pub tfa_secret: Option<String>,
    pub status: String,
    pub role: Option<uuid::Uuid>,
    pub token: Option<String>,
    pub last_access: Option<chrono::DateTime<chrono::Utc>>,
    pub last_page: Option<String>,
    pub provider: String,
    pub external_identifier: Option<String>,
    pub auth_data: Option<serde_json::Value>,
    pub email_notifications: Option<bool>,
    pub appearance: Option<String>,
    pub theme_dark: Option<String>,
    pub theme_light: Option<String>,
    pub theme_light_overrides: Option<serde_json::Value>,
    pub theme_dark_overrides: Option<serde_json::Value>,
}
impl DirectusUser {
    #[cfg(feature = "postgres")]
    pub async fn role(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(role) = self.role.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::table()
            .find(role)
            .first::<
                crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_role(
        conn: &mut diesel_async::AsyncPgConnection,
        role: &crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::role
                    .eq(&role.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_email(
        email: Option<&str>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::directus_users::directus_users::email,
                    email,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_external_identifier(
        external_identifier: Option<&str>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::directus_users::directus_users::external_identifier,
                    external_identifier,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_token(
        token: Option<&str>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::directus_users::directus_users::token,
                    token,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
