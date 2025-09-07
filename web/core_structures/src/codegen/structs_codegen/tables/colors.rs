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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::colors::colors)]
pub struct Color {
    pub name: String,
    pub hexadecimal_value: String,
    pub description: String,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for Color {
    const TABLE_NAME: &'static str = "colors";
}
impl web_common_traits::prelude::ExtensionTable<crate::Color> for Color where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>
{
}
impl diesel::Identifiable for Color {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Color {
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::colors::colors;
        Self::table().filter(colors::name.eq(name)).order_by(colors::id.asc()).first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_hexadecimal_value(
        hexadecimal_value: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::colors::colors;
        Self::table()
            .filter(colors::hexadecimal_value.eq(hexadecimal_value))
            .order_by(colors::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::colors::colors;
        Self::table()
            .filter(colors::description.eq(description))
            .order_by(colors::id.asc())
            .first::<Self>(conn)
    }
}
impl AsRef<Color> for Color {
    fn as_ref(&self) -> &Color {
        self
    }
}
