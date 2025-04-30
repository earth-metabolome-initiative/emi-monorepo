#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::email_providers::EmailProvider
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::table,
            )
            .values(self)
            .on_conflict((
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::email_id,
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::login_provider_id,
            ))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::email_providers::EmailProvider
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::table,
            )
            .values(self)
            .on_conflict((
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::email_id,
                crate::codegen::diesel_codegen::tables::email_providers::email_providers::login_provider_id,
            ))
            .do_nothing()
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
