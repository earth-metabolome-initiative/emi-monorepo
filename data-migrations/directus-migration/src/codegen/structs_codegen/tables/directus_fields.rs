#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_fields::directus_fields
)]
pub struct DirectusField {
    pub id: i32,
    pub collection: String,
    pub field: String,
    pub special: Option<String>,
    pub interface: Option<String>,
    pub options: Option<serde_json::Value>,
    pub display: Option<String>,
    pub display_options: Option<serde_json::Value>,
    pub readonly: bool,
    pub hidden: bool,
    pub sort: Option<i32>,
    pub width: Option<String>,
    pub translations: Option<serde_json::Value>,
    pub note: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub required: Option<bool>,
    pub group: Option<String>,
    pub validation: Option<serde_json::Value>,
    pub validation_message: Option<String>,
}
impl diesel::Identifiable for DirectusField {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusField {}
