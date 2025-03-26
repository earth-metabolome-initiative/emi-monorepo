#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_translations::directus_translations
)]
pub struct DirectusTranslation {
    pub id: uuid::Uuid,
    pub language: String,
    pub key: String,
    pub value: String,
}
impl DirectusTranslation {}
