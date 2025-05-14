#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::document_formats::DocumentFormat
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::document_formats::document_formats::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                extension
                    .ne(excluded(extension))
                    .or(mime_type.ne(excluded(mime_type)))
                    .or(description.ne(excluded(description)))
                    .or(icon.ne(excluded(icon)))
                    .or(color.ne(excluded(color))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
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
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::document_formats::document_formats::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                extension
                    .ne(excluded(extension))
                    .or(mime_type.ne(excluded(mime_type)))
                    .or(description.ne(excluded(description)))
                    .or(icon.ne(excluded(icon)))
                    .or(color.ne(excluded(color))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
