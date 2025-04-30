#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::colors::Color
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(crate::codegen::diesel_codegen::tables::colors::colors::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::colors::colors::id)
            .do_update()
            .set(self)
            .filter(diesel::BoolExpressionMethods::and(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::colors::colors::name.ne(
                        diesel::upsert::excluded(
                            crate::codegen::diesel_codegen::tables::colors::colors::name,
                        ),
                    ),
                    crate::codegen::diesel_codegen::tables::colors::colors::hexadecimal_value
                        .ne(diesel::upsert::excluded(
                        crate::codegen::diesel_codegen::tables::colors::colors::hexadecimal_value,
                    )),
                ),
                crate::codegen::diesel_codegen::tables::colors::colors::description.ne(
                    diesel::upsert::excluded(
                        crate::codegen::diesel_codegen::tables::colors::colors::description,
                    ),
                ),
            ))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::colors::Color
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(crate::codegen::diesel_codegen::tables::colors::colors::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::colors::colors::id)
            .do_update()
            .set(self)
            .filter(diesel::BoolExpressionMethods::and(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::colors::colors::name.ne(
                        diesel::upsert::excluded(
                            crate::codegen::diesel_codegen::tables::colors::colors::name,
                        ),
                    ),
                    crate::codegen::diesel_codegen::tables::colors::colors::hexadecimal_value
                        .ne(diesel::upsert::excluded(
                        crate::codegen::diesel_codegen::tables::colors::colors::hexadecimal_value,
                    )),
                ),
                crate::codegen::diesel_codegen::tables::colors::colors::description.ne(
                    diesel::upsert::excluded(
                        crate::codegen::diesel_codegen::tables::colors::colors::description,
                    ),
                ),
            ))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
