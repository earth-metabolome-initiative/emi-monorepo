#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_relations::directus_relations
)]
pub struct DirectusRelation {
    pub id: i32,
    pub many_collection: String,
    pub many_field: String,
    pub one_collection: Option<String>,
    pub one_field: Option<String>,
    pub one_collection_field: Option<String>,
    pub one_allowed_collections: Option<String>,
    pub junction_field: Option<String>,
    pub sort_field: Option<String>,
    pub one_deselect_action: String,
}
impl DirectusRelation {}
