#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                measured_with_model
                    .ne(excluded(measured_with_model))
                    .or(
                        procedure_template_measured_with_model
                            .ne(excluded(procedure_template_measured_with_model)),
                    )
                    .or(poured_from_model.ne(excluded(poured_from_model)))
                    .or(
                        procedure_template_poured_from_model
                            .ne(excluded(procedure_template_poured_from_model)),
                    )
                    .or(poured_into_model.ne(excluded(poured_into_model)))
                    .or(
                        procedure_template_poured_into_model
                            .ne(excluded(procedure_template_poured_into_model)),
                    )
                    .or(liters.ne(excluded(liters))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                measured_with_model
                    .ne(excluded(measured_with_model))
                    .or(
                        procedure_template_measured_with_model
                            .ne(excluded(procedure_template_measured_with_model)),
                    )
                    .or(poured_from_model.ne(excluded(poured_from_model)))
                    .or(
                        procedure_template_poured_from_model
                            .ne(excluded(procedure_template_poured_from_model)),
                    )
                    .or(poured_into_model.ne(excluded(poured_into_model)))
                    .or(
                        procedure_template_poured_into_model
                            .ne(excluded(procedure_template_poured_into_model)),
                    )
                    .or(liters.ne(excluded(liters))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
