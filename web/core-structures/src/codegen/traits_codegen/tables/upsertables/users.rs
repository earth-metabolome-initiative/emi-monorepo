#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::users::User
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(crate::codegen::diesel_codegen::tables::users::users::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::users::users::id)
            .do_update()
            .set(self)
            .filter(diesel::BoolExpressionMethods::and(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        crate::codegen::diesel_codegen::tables::users::users::first_name.ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::users::users::first_name,
                            ),
                        ),
                        crate::codegen::diesel_codegen::tables::users::users::last_name.ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::users::users::last_name,
                            ),
                        ),
                    ),
                    crate::codegen::diesel_codegen::tables::users::users::created_at.ne(
                        diesel::upsert::excluded(
                            crate::codegen::diesel_codegen::tables::users::users::created_at,
                        ),
                    ),
                ),
                crate::codegen::diesel_codegen::tables::users::users::updated_at.ne(
                    diesel::upsert::excluded(
                        crate::codegen::diesel_codegen::tables::users::users::updated_at,
                    ),
                ),
            ))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::users::User
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(crate::codegen::diesel_codegen::tables::users::users::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::users::users::id)
            .do_update()
            .set(self)
            .filter(diesel::BoolExpressionMethods::and(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        crate::codegen::diesel_codegen::tables::users::users::first_name.ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::users::users::first_name,
                            ),
                        ),
                        crate::codegen::diesel_codegen::tables::users::users::last_name.ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::users::users::last_name,
                            ),
                        ),
                    ),
                    crate::codegen::diesel_codegen::tables::users::users::created_at.ne(
                        diesel::upsert::excluded(
                            crate::codegen::diesel_codegen::tables::users::users::created_at,
                        ),
                    ),
                ),
                crate::codegen::diesel_codegen::tables::users::users::updated_at.ne(
                    diesel::upsert::excluded(
                        crate::codegen::diesel_codegen::tables::users::users::updated_at,
                    ),
                ),
            ))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
