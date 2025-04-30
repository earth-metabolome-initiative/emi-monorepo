#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::user_organizations::UserOrganization
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::table,
            )
            .values(self)
            .on_conflict((
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::user_id,
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::organization_id,
            ))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::user_organizations::UserOrganization
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::table,
            )
            .values(self)
            .on_conflict((
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::user_id,
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::organization_id,
            ))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
