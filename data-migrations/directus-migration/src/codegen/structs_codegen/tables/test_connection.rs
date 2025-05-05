#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::test_connection::test_connection
)]
pub struct TestConnection {
    pub id: i32,
}
impl diesel::Identifiable for TestConnection {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl TestConnection {}
