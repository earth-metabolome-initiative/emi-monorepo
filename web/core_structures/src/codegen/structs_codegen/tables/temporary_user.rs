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
impl web_common_traits::prelude::TableName for TemporaryUser {
    const TABLE_NAME: &'static str = "temporary_user";
}
impl diesel::Identifiable for TemporaryUser {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl TemporaryUser {
    pub fn login_provider<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::login_providers::LoginProvider::table(),
                self.login_provider_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_email_and_login_provider_id(
        email: &str,
        login_provider_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::temporary_user::temporary_user;
        use diesel::BoolExpressionMethods;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(
                temporary_user::email
                    .eq(email)
                    .and(temporary_user::login_provider_id.eq(login_provider_id)),
            )
            .order_by(temporary_user::id.asc())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_email(
        email: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::temporary_user::temporary_user;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(temporary_user::email.eq(email))
            .order_by(temporary_user::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_login_provider_id(
        login_provider_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::temporary_user::temporary_user;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(temporary_user::login_provider_id.eq(login_provider_id))
            .order_by(temporary_user::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<TemporaryUser> for TemporaryUser {
    fn as_ref(&self) -> &TemporaryUser {
        self
    }
}
