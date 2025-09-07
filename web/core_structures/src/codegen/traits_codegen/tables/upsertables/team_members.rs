#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection> for crate::TeamMember {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::team_members::team_members::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((team_id, member_id))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection> for crate::TeamMember {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::team_members::team_members::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((team_id, member_id))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
