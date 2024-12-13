#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Queryable, diesel::Selectable))]
pub struct CompositeUsers {
    pub primary_id: i32,
    pub secondary_id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
impl CompositeUsers {
    #[cfg(feature = "diesel")]
    pub fn primary(&self, conn: &mut PgConnection) -> Result<Users, DieselError> {
        use crate::schema::users;
        users::table.filter(users::id.eq(self.primary_id)).first::<Users>(conn)
    }
    #[cfg(feature = "diesel")]
    pub fn secondary(&self, conn: &mut PgConnection) -> Result<Users, DieselError> {
        use crate::schema::users;
        users::table
            .filter(users::id.eq(self.secondary_id))
            .first::<Users>(conn)
    }
}
