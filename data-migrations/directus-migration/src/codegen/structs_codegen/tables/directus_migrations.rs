#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(version))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_migrations::directus_migrations
)]
pub struct DirectusMigration {
    pub version: String,
    pub name: String,
    pub timestamp: Option<rosetta_timestamp::TimestampUTC>,
}
impl DirectusMigration {}
