#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::organizations::organizations
)]
pub struct Organization {
    pub name: String,
    pub url: String,
    pub country: String,
    pub alpha_two_code: String,
    pub state_province: Option<String>,
    pub domain: String,
    pub id: i16,
}
impl Organization {
    #[cfg(feature = "postgres")]
    pub async fn from_url(
        url: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::organizations::organizations::url,
                url,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_domain(
        domain: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::organizations::organizations::domain,
                domain,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
