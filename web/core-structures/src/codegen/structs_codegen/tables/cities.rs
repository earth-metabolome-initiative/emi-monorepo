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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::cities::cities)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub iso: iso_codes::CountryCode,
}
impl diesel::Identifiable for City {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl City {
    #[cfg(feature = "postgres")]
    pub async fn iso(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::countries::Country, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::countries::Country::table()
            .filter(
                crate::codegen::diesel_codegen::tables::countries::countries::dsl::iso
                    .eq(&self.iso),
            )
            .first::<crate::codegen::structs_codegen::tables::countries::Country>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_iso(
        conn: &mut diesel_async::AsyncPgConnection,
        iso: &crate::codegen::structs_codegen::tables::countries::Country,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(crate::codegen::diesel_codegen::tables::cities::cities::dsl::iso.eq(iso.iso))
            .load::<Self>(conn)
            .await
    }
}
