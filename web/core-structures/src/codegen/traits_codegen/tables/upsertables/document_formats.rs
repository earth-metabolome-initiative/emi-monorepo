#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::document_formats::DocumentFormat
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::document_formats::document_formats::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::document_formats::document_formats::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                crate::codegen::diesel_codegen::tables::document_formats::document_formats::extension
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::document_formats::document_formats::extension,
                                        ),
                                    ),
                                crate::codegen::diesel_codegen::tables::document_formats::document_formats::mime_type
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::document_formats::document_formats::mime_type,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::document_formats::document_formats::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::document_formats::document_formats::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::document_formats::document_formats::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::document_formats::document_formats::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::document_formats::document_formats::color
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::document_formats::document_formats::color,
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
    for crate::codegen::structs_codegen::tables::document_formats::DocumentFormat
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::document_formats::document_formats::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::document_formats::document_formats::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                crate::codegen::diesel_codegen::tables::document_formats::document_formats::extension
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::document_formats::document_formats::extension,
                                        ),
                                    ),
                                crate::codegen::diesel_codegen::tables::document_formats::document_formats::mime_type
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::document_formats::document_formats::mime_type,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::document_formats::document_formats::description
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::document_formats::document_formats::description,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::document_formats::document_formats::icon
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::document_formats::document_formats::icon,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::document_formats::document_formats::color
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::document_formats::document_formats::color,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
