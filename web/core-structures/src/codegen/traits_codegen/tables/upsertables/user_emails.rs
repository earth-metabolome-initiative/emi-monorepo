#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::user_emails::UserEmail
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::user_emails::user_emails::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::user_emails::user_emails::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::user_emails::user_emails::email
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::user_emails::user_emails::email,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_by
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_by,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::user_emails::user_emails::primary_email
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::user_emails::user_emails::primary_email,
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
    for crate::codegen::structs_codegen::tables::user_emails::UserEmail
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::user_emails::user_emails::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::user_emails::user_emails::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::user_emails::user_emails::email
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::user_emails::user_emails::email,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_by
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_by,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::user_emails::user_emails::created_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::user_emails::user_emails::primary_email
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::user_emails::user_emails::primary_email,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
