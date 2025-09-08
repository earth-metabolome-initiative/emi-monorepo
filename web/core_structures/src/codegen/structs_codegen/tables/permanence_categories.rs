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
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories
)]
pub struct PermanenceCategory {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for PermanenceCategory {
    const TABLE_NAME: &'static str = "permanence_categories";
}
impl<'a> From<&'a PermanenceCategory>
    for web_common_traits::database::IdOrBuilder<
        i16,
        crate::codegen::structs_codegen::tables::insertables::InsertablePermanenceCategoryBuilder,
    >
{
    fn from(value: &'a PermanenceCategory) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl web_common_traits::prelude::ExtensionTable<crate::PermanenceCategory> for PermanenceCategory where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>
{
}
impl diesel::Identifiable for PermanenceCategory {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl PermanenceCategory {
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

        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        Self::table()
            .filter(permanence_categories::name.eq(name))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        Self::table()
            .filter(permanence_categories::description.eq(description))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        Self::table()
            .filter(permanence_categories::icon.eq(icon))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_color_id(
        color_id: i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        Self::table()
            .filter(permanence_categories::color_id.eq(color_id))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
    }
}
impl AsRef<PermanenceCategory> for PermanenceCategory {
    fn as_ref(&self) -> &PermanenceCategory {
        self
    }
}
