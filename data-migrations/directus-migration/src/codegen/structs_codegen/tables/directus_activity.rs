#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_activity::directus_activity
)]
pub struct DirectusActivity {
    pub id: i32,
    pub action: String,
    pub user: Option<rosetta_uuid::Uuid>,
    pub timestamp: rosetta_timestamp::TimestampUTC,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub collection: String,
    pub item: String,
    pub comment: Option<String>,
    pub origin: Option<String>,
}
impl DirectusActivity {}
