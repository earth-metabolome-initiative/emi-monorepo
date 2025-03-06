#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: users :: users))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub picture: Vec<u8>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl User {}
