#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_policies::directus_policies
)]
pub struct DirectusPolicy {
    pub id: uuid::Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub ip_access: Option<String>,
    pub enforce_tfa: bool,
    pub admin_access: bool,
    pub app_access: bool,
}
impl DirectusPolicy {}
