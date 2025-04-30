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
    table_name = crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions
)]
pub struct DirectusExtension {
    pub enabled: bool,
    pub id: rosetta_uuid::Uuid,
    pub folder: String,
    pub source: String,
    pub bundle: Option<rosetta_uuid::Uuid>,
}
impl diesel::Identifiable for DirectusExtension {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusExtension {}
