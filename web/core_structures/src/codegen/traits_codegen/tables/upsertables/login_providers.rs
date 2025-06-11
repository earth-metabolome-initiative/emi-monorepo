#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::login_providers::LoginProvider
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::login_providers::login_providers::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(icon.ne(excluded(icon)))
                    .or(client_id.ne(excluded(client_id)))
                    .or(redirect_uri.ne(excluded(redirect_uri)))
                    .or(oauth_url.ne(excluded(oauth_url)))
                    .or(scope.ne(excluded(scope))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::login_providers::LoginProvider
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::login_providers::login_providers::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(icon.ne(excluded(icon)))
                    .or(client_id.ne(excluded(client_id)))
                    .or(redirect_uri.ne(excluded(redirect_uri)))
                    .or(oauth_url.ne(excluded(oauth_url)))
                    .or(scope.ne(excluded(scope))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
