#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::addresses::addresses)]
pub struct Address {
    pub id: i32,
    pub country: String,
    pub city: String,
    pub street: String,
    pub street_number: String,
    pub postal_code: String,
    pub geolocation: postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>,
    pub city_code: String,
}
impl Address {}
