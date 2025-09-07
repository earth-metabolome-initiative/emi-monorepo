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
    pub alpha_two_code: ::iso_codes::CountryCode,
    pub state_province: Option<String>,
    pub domain: String,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for Organization {
    const TABLE_NAME: &'static str = "organizations";
}
impl web_common_traits::prelude::ExtensionTable<crate::Organization> for Organization where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>
{
}
impl diesel::Identifiable for Organization {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Organization {
    #[cfg(feature = "postgres")]
    pub fn from_url(
        url: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::organizations::organizations;
        Self::table()
            .filter(organizations::url.eq(url))
            .order_by(organizations::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_domain(
        domain: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::organizations::organizations;
        Self::table()
            .filter(organizations::domain.eq(domain))
            .order_by(organizations::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::organizations::organizations;
        Self::table()
            .filter(organizations::name.eq(name))
            .order_by(organizations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_country(
        country: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::organizations::organizations;
        Self::table()
            .filter(organizations::country.eq(country))
            .order_by(organizations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_alpha_two_code(
        alpha_two_code: &::iso_codes::CountryCode,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::organizations::organizations;
        Self::table()
            .filter(organizations::alpha_two_code.eq(alpha_two_code))
            .order_by(organizations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_state_province(
        state_province: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::organizations::organizations;
        Self::table()
            .filter(organizations::state_province.eq(state_province))
            .order_by(organizations::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<Organization> for Organization {
    fn as_ref(&self) -> &Organization {
        self
    }
}
