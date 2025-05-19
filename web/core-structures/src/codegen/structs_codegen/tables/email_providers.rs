#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(email_id, login_provider_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::email_providers::email_providers
)]
pub struct EmailProvider {
    pub email_id: i32,
    pub login_provider_id: i16,
}
impl diesel::Identifiable for EmailProvider {
    type Id = (i32, i16);
    fn id(self) -> Self::Id {
        (self.email_id, self.login_provider_id)
    }
}
impl EmailProvider {
    #[cfg(feature = "postgres")]
    pub async fn email(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::user_emails::UserEmail::table()
            .filter(
                crate::codegen::diesel_codegen::tables::user_emails::user_emails::dsl::id
                    .eq(&self.email_id),
            )
            .first::<crate::codegen::structs_codegen::tables::user_emails::UserEmail>(conn)
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider::table()
            .filter(
                crate::codegen::diesel_codegen::tables::login_providers::login_providers::dsl::id
                    .eq(&self.login_provider_id),
            )
            .first::<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_email_id(
        conn: &mut diesel_async::AsyncPgConnection,
        email_id: &crate::codegen::structs_codegen::tables::user_emails::UserEmail,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::dsl::email_id
                    .eq(email_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_login_provider_id(
        conn: &mut diesel_async::AsyncPgConnection,
        login_provider_id: &crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::dsl::login_provider_id
                    .eq(login_provider_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_email_id_and_login_provider_id(
        email_id: &i32,
        login_provider_id: &i16,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
            associations::HasTable,
        };
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::email_id
                    .eq(email_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::email_providers::email_providers::login_provider_id
                            .eq(login_provider_id),
                    ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
