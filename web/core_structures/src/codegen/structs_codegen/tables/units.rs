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
#[diesel(belongs_to(crate::Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::units::units)]
pub struct Unit {
    pub name: String,
    pub unit: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for Unit {
    const TABLE_NAME: &'static str = "units";
}
impl<'a> From<&'a Unit>
    for web_common_traits::database::IdOrBuilder<
        i16,
        crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder,
    >
{
    fn from(value: &'a Unit) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl web_common_traits::prelude::ExtensionTable<crate::Unit> for Unit where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>
{
}
impl diesel::Identifiable for Unit {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Unit {
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Color, diesel::result::Error>
    where
        crate::Color: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Color::read(self.color_id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::units::units;
        Self::table().filter(units::name.eq(name)).order_by(units::id.asc()).first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_unit(
        unit: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::units::units;
        Self::table().filter(units::unit.eq(unit)).order_by(units::id.asc()).first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::units::units;
        Self::table().filter(units::icon.eq(icon)).order_by(units::id.asc()).load::<Self>(conn)
    }
}
impl AsRef<Unit> for Unit {
    fn as_ref(&self) -> &Unit {
        self
    }
}
