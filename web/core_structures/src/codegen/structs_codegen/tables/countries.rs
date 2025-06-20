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
#[diesel(primary_key(iso))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::countries::countries)]
pub struct Country {
    pub iso: ::iso_codes::CountryCode,
    pub name: String,
}
impl web_common_traits::prelude::TableName for Country {
    const TABLE_NAME: &'static str = "countries";
}
impl diesel::Identifiable for Country {
    type Id = ::iso_codes::CountryCode;
    fn id(self) -> Self::Id {
        self.iso
    }
}
impl Country {
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::countries::countries;
        Self::table()
            .filter(countries::name.eq(name))
            .order_by(countries::iso.asc())
            .first::<Self>(conn)
            .optional()
    }
}
impl AsRef<Country> for Country {
    fn as_ref(&self) -> &Country {
        self
    }
}
