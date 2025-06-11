#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::centrifuge_procedure_models::centrifuge_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                seconds
                    .ne(excluded(seconds))
                    .or(rotation_per_minute.ne(excluded(rotation_per_minute)))
                    .or(centrifuged_with.ne(excluded(centrifuged_with)))
                    .or(container_id.ne(excluded(container_id))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::centrifuge_procedure_models::centrifuge_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                seconds
                    .ne(excluded(seconds))
                    .or(rotation_per_minute.ne(excluded(rotation_per_minute)))
                    .or(centrifuged_with.ne(excluded(centrifuged_with)))
                    .or(container_id.ne(excluded(container_id))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
