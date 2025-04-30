#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(iso))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::countries::countries)]
pub struct Country {
    pub iso: iso_codes::CountryCode,
    pub name: String,
}
impl diesel::Identifiable for Country {
    type Id = iso_codes::CountryCode;
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
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::countries::countries::name,
                name,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
