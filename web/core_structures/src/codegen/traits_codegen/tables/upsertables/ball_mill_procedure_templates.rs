#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                kelvin
                    .ne(excluded(kelvin))
                    .or(
                        kelvin_tolerance_percentage
                            .ne(excluded(kelvin_tolerance_percentage)),
                    )
                    .or(seconds.ne(excluded(seconds)))
                    .or(hertz.ne(excluded(hertz)))
                    .or(bead_model.ne(excluded(bead_model_id)))
                    .or(
                        procedure_template_bead_model
                            .ne(excluded(procedure_template_bead_model_id)),
                    )
                    .or(number_of_beads.ne(excluded(number_of_beads)))
                    .or(milled_with_model.ne(excluded(milled_with_model_id)))
                    .or(
                        procedure_template_milled_with_model
                            .ne(excluded(procedure_template_milled_with_model_id)),
                    )
                    .or(milled_container_model.ne(excluded(milled_container_model_id)))
                    .or(
                        procedure_template_milled_container_model
                            .ne(excluded(procedure_template_milled_container_model_id)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                kelvin
                    .ne(excluded(kelvin))
                    .or(
                        kelvin_tolerance_percentage
                            .ne(excluded(kelvin_tolerance_percentage)),
                    )
                    .or(seconds.ne(excluded(seconds)))
                    .or(hertz.ne(excluded(hertz)))
                    .or(bead_model.ne(excluded(bead_model_id)))
                    .or(
                        procedure_template_bead_model
                            .ne(excluded(procedure_template_bead_model_id)),
                    )
                    .or(number_of_beads.ne(excluded(number_of_beads)))
                    .or(milled_with_model.ne(excluded(milled_with_model_id)))
                    .or(
                        procedure_template_milled_with_model
                            .ne(excluded(procedure_template_milled_with_model_id)),
                    )
                    .or(milled_container_model.ne(excluded(milled_container_model_id)))
                    .or(
                        procedure_template_milled_container_model
                            .ne(excluded(procedure_template_milled_container_model_id)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
