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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::users::users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub created_at: rosetta_timestamp::TimestampUTC,
    pub updated_at: rosetta_timestamp::TimestampUTC,
}
impl diesel::Identifiable for User {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl User {}
