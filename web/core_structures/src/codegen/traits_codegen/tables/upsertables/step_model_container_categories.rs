#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                step_model_id
                    .ne(excluded(step_model_id))
                    .or(container_category.ne(excluded(container_category)))
                    .or(expected_kelvin.ne(excluded(expected_kelvin)))
                    .or(tolerance_kelvin.ne(excluded(tolerance_kelvin)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                step_model_id
                    .ne(excluded(step_model_id))
                    .or(container_category.ne(excluded(container_category)))
                    .or(expected_kelvin.ne(excluded(expected_kelvin)))
                    .or(tolerance_kelvin.ne(excluded(tolerance_kelvin)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
