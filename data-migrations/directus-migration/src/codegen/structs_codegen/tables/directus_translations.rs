#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_translations::directus_translations
)]
pub struct DirectusTranslation {
    pub id: rosetta_uuid::Uuid,
    pub language: String,
    pub key: String,
    pub value: String,
}
impl diesel::Identifiable for DirectusTranslation {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusTranslation {}
