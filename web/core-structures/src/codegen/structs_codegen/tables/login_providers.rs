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
    table_name = crate::codegen::diesel_codegen::tables::login_providers::login_providers
)]
pub struct LoginProvider {
    pub id: i16,
    pub name: String,
    pub icon: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub oauth_url: String,
    pub scope: String,
}
impl diesel::Identifiable for LoginProvider {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl LoginProvider {
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::login_providers::login_providers::name
                    .eq(name),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_icon(
        icon: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::login_providers::login_providers;
        Self::table()
            .filter(login_providers::icon.eq(icon))
            .order_by(login_providers::id.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_client_id(
        client_id: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::login_providers::login_providers;
        Self::table()
            .filter(login_providers::client_id.eq(client_id))
            .order_by(login_providers::id.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_redirect_uri(
        redirect_uri: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::login_providers::login_providers;
        Self::table()
            .filter(login_providers::redirect_uri.eq(redirect_uri))
            .order_by(login_providers::id.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_oauth_url(
        oauth_url: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::login_providers::login_providers;
        Self::table()
            .filter(login_providers::oauth_url.eq(oauth_url))
            .order_by(login_providers::id.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_scope(
        scope: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::login_providers::login_providers;
        Self::table()
            .filter(login_providers::scope.eq(scope))
            .order_by(login_providers::id.asc())
            .load::<Self>(conn)
            .await
    }
}
