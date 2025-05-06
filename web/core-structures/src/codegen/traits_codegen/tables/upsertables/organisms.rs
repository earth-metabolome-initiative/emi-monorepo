#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::organisms::Organism
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::organisms::organisms::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::organisms::organisms::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::organisms::organisms::name
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::organisms::organisms::name,
                            ),
                        ),
                    crate::codegen::diesel_codegen::tables::organisms::organisms::nameplate_category
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::organisms::organisms::nameplate_category,
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
    for crate::codegen::structs_codegen::tables::organisms::Organism
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::organisms::organisms::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::organisms::organisms::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::organisms::organisms::name
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::organisms::organisms::name,
                            ),
                        ),
                    crate::codegen::diesel_codegen::tables::organisms::organisms::nameplate_category
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::organisms::organisms::nameplate_category,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
