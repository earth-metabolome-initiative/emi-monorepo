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
    pub iso: ::iso_codes::CountryCode,
}
impl web_common_traits::prelude::TableName for City {
    const TABLE_NAME: &'static str = "cities";
}
impl diesel::Identifiable for City {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl City {
    pub fn iso<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::countries::Country,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::countries::Country: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::countries::Country as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::countries::Country as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::countries::Country as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::countries::Country as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::countries::Country as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::countries::Country as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::countries::Country,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::countries::Country::table(),
                self.iso,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::cities::cities;
        Self::table().filter(cities::name.eq(name)).order_by(cities::id.asc()).load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_iso(
        iso: &::iso_codes::CountryCode,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::cities::cities;
        Self::table().filter(cities::iso.eq(iso)).order_by(cities::id.asc()).load::<Self>(conn)
    }
}
impl AsRef<City> for City {
    fn as_ref(&self) -> &City {
        self
    }
}
