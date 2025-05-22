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
impl diesel::Identifiable for Country {
    type Id = ::iso_codes::CountryCode;
    fn id(self) -> Self::Id {
        self.iso
    }
}
impl Country {
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(crate::codegen::diesel_codegen::tables::countries::countries::name.eq(name))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
