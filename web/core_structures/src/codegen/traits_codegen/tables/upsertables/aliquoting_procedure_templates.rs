#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                liters
                    .ne(excluded(liters))
                    .or(aliquoted_from_model.ne(excluded(aliquoted_from_model)))
                    .or(
                        procedure_template_aliquoted_from_model
                            .ne(excluded(procedure_template_aliquoted_from_model)),
                    )
                    .or(aliquoted_into_model.ne(excluded(aliquoted_into_model)))
                    .or(
                        procedure_template_aliquoted_into_model
                            .ne(excluded(procedure_template_aliquoted_into_model)),
                    )
                    .or(aliquoted_with_model.ne(excluded(aliquoted_with_model)))
                    .or(
                        procedure_template_aliquoted_with_model
                            .ne(excluded(procedure_template_aliquoted_with_model)),
                    )
                    .or(pipette_tip_model.ne(excluded(pipette_tip_model)))
                    .or(
                        procedure_template_pipette_tip_model
                            .ne(excluded(procedure_template_pipette_tip_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                liters
                    .ne(excluded(liters))
                    .or(aliquoted_from_model.ne(excluded(aliquoted_from_model)))
                    .or(
                        procedure_template_aliquoted_from_model
                            .ne(excluded(procedure_template_aliquoted_from_model)),
                    )
                    .or(aliquoted_into_model.ne(excluded(aliquoted_into_model)))
                    .or(
                        procedure_template_aliquoted_into_model
                            .ne(excluded(procedure_template_aliquoted_into_model)),
                    )
                    .or(aliquoted_with_model.ne(excluded(aliquoted_with_model)))
                    .or(
                        procedure_template_aliquoted_with_model
                            .ne(excluded(procedure_template_aliquoted_with_model)),
                    )
                    .or(pipette_tip_model.ne(excluded(pipette_tip_model)))
                    .or(
                        procedure_template_pipette_tip_model
                            .ne(excluded(procedure_template_pipette_tip_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
