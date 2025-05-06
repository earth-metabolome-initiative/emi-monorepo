#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::tool_categories::ToolCategory
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::name
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::name,
                                ),
                            ),
                        crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::description
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::description,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::icon
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::icon,
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
    for crate::codegen::structs_codegen::tables::tool_categories::ToolCategory
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::name
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::name,
                                ),
                            ),
                        crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::description
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::description,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::icon
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::icon,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
