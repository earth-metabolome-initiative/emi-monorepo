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
    table_name = crate::codegen::diesel_codegen::tables::organizations::organizations
)]
pub struct Organization {
    pub name: String,
    pub url: String,
    pub country: String,
    pub alpha_two_code: iso_codes::CountryCode,
    pub state_province: Option<String>,
    pub domain: String,
    pub id: i16,
}
impl diesel::Identifiable for Organization {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Organization {
    #[cfg(feature = "postgres")]
    pub async fn from_url(
        url: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organizations::organizations::url.eq(url),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_domain(
        domain: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organizations::organizations::domain
                    .eq(domain),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
