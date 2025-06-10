#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::colors::Color
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::colors::colors::*;
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(hexadecimal_value.ne(excluded(hexadecimal_value)))
                    .or(description.ne(excluded(description))),
            )
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
        use crate::codegen::diesel_codegen::tables::colors::colors::*;
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(hexadecimal_value.ne(excluded(hexadecimal_value)))
                    .or(description.ne(excluded(description))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
