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
impl web_common_traits::prelude::TableName for TestConnection {
    const TABLE_NAME: &'static str = "Test_Connection";
}
impl diesel::Identifiable for TestConnection {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl TestConnection {}
impl AsRef<TestConnection> for TestConnection {
    fn as_ref(&self) -> &TestConnection {
        self
    }
}
