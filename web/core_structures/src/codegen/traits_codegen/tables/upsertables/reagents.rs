#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::reagents::Reagent
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::reagents::reagents::*;
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
                purity
                    .ne(excluded(purity))
                    .or(cas_code.ne(excluded(cas_code)))
                    .or(molecular_formula.ne(excluded(molecular_formula))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::reagents::Reagent
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::reagents::reagents::*;
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
                purity
                    .ne(excluded(purity))
                    .or(cas_code.ne(excluded(cas_code)))
                    .or(molecular_formula.ne(excluded(molecular_formula))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
