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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::ranks::ranks)]
pub struct Rank {
    pub name: String,
    pub description: String,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for Rank {
    const TABLE_NAME: &'static str = "ranks";
}
impl diesel::Identifiable for Rank {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Rank {}
impl AsRef<Rank> for Rank {
    fn as_ref(&self) -> &Rank {
        self
    }
}
