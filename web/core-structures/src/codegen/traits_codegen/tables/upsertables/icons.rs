#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::icons::Icon {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::RunQueryDsl;
        diesel::insert_into(crate::codegen::diesel_codegen::tables::icons::icons::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::icons::icons::id)
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::icons::icons::name
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::icons::icons::name,
                            ),
                        ),
                    crate::codegen::diesel_codegen::tables::icons::icons::description
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::icons::icons::description,
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
for crate::codegen::structs_codegen::tables::icons::Icon {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::RunQueryDsl;
        diesel::insert_into(crate::codegen::diesel_codegen::tables::icons::icons::table)
            .values(self)
            .on_conflict(crate::codegen::diesel_codegen::tables::icons::icons::id)
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::icons::icons::name
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::icons::icons::name,
                            ),
                        ),
                    crate::codegen::diesel_codegen::tables::icons::icons::description
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::icons::icons::description,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
