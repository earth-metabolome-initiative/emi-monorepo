#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::login_providers::LoginProvider
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::login_providers::login_providers::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::login_providers::login_providers::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::name
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::name,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::icon
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::icon,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::client_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::login_providers::login_providers::client_id,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::login_providers::login_providers::redirect_uri
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::login_providers::login_providers::redirect_uri,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::login_providers::login_providers::oauth_url
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::oauth_url,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::scope
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::scope,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
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
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::login_providers::login_providers::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::login_providers::login_providers::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::name
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::name,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::icon
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::icon,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::client_id
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::login_providers::login_providers::client_id,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::login_providers::login_providers::redirect_uri
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::login_providers::login_providers::redirect_uri,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::login_providers::login_providers::oauth_url
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::oauth_url,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::login_providers::login_providers::scope
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::login_providers::login_providers::scope,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
