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
impl diesel::Identifiable for Color {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Color {}
impl AsRef<Color> for Color {
    fn as_ref(&self) -> &Color {
        self
    }
}
