#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
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
impl DirectusExtension {}
