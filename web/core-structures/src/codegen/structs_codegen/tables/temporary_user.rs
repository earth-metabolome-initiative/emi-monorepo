#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::temporary_user::temporary_user
)]
pub struct TemporaryUser {
    pub id: i32,
    pub email: String,
    pub login_provider_id: i16,
}
impl diesel::Identifiable for TemporaryUser {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl TemporaryUser {
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
    pub async fn from_login_provider_id(
        conn: &mut diesel_async::AsyncPgConnection,
        login_provider_id: &crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::temporary_user::temporary_user::dsl::login_provider_id
                    .eq(login_provider_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_email_and_login_provider_id(
        email: &str,
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
                crate::codegen::diesel_codegen::tables::temporary_user::temporary_user::email
                    .eq(email)
                    .and(
                        crate::codegen::diesel_codegen::tables::temporary_user::temporary_user::login_provider_id
                            .eq(login_provider_id),
                    ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_email(
        email: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::temporary_user::temporary_user;
        Self::table()
            .filter(temporary_user::email.eq(email))
            .order_by(temporary_user::id.asc())
            .load::<Self>(conn)
            .await
    }
}
