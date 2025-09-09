#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::countries::Country,
        foreign_key = iso
    )
)]
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
impl<'a> From<&'a City>
    for web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder,
    >
{
    fn from(value: &'a City) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::cities::City,
    > for City
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
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
    ) -> Result<crate::codegen::structs_codegen::tables::countries::Country, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::countries::Country:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::countries::Country::read(self.iso, conn)
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
}
impl AsRef<City> for City {
    fn as_ref(&self) -> &City {
        self
    }
}
