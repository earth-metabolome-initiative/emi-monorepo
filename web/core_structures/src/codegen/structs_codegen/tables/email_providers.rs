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
impl web_common_traits::prelude::TableName for EmailProvider {
    const TABLE_NAME: &'static str = "email_providers";
}
impl diesel::Identifiable for EmailProvider {
    type Id = (i32, i16);
    fn id(self) -> Self::Id {
        (self.email_id, self.login_provider_id)
    }
}
impl EmailProvider {
    pub fn email<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::user_emails::UserEmail: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::user_emails::UserEmail as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::user_emails::UserEmail::table(),
                self.email_id,
            ),
            conn,
        )
    }
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::login_providers::LoginProvider::table(),
                self.login_provider_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_email_id(
        email_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::email_providers::email_providers;
        Self::table()
            .filter(email_providers::email_id.eq(email_id))
            .order_by((email_providers::email_id.asc(), email_providers::login_provider_id.asc()))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_login_provider_id(
        login_provider_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::email_providers::email_providers;
        Self::table()
            .filter(email_providers::login_provider_id.eq(login_provider_id))
            .order_by((email_providers::email_id.asc(), email_providers::login_provider_id.asc()))
            .load::<Self>(conn)
    }
}
impl AsRef<EmailProvider> for EmailProvider {
    fn as_ref(&self) -> &EmailProvider {
        self
    }
}
