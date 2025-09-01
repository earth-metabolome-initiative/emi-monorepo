#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
            .do_update()
            .set(self)
            .filter(
                liters
                    .ne(excluded(liters))
                    .or(stratified_source_model.ne(excluded(stratified_source_model)))
                    .or(
                        foreign_procedure_template
                            .ne(excluded(foreign_procedure_template)),
                    )
                    .or(
                        procedure_template_stratified_source_model
                            .ne(excluded(procedure_template_stratified_source_model)),
                    )
                    .or(
                        supernatant_destination_model
                            .ne(excluded(supernatant_destination_model)),
                    )
                    .or(
                        procedure_template_supernatant_destination_model
                            .ne(
                                excluded(procedure_template_supernatant_destination_model),
                            ),
                    )
                    .or(transferred_with_model.ne(excluded(transferred_with_model)))
                    .or(
                        procedure_template_transferred_with_model
                            .ne(excluded(procedure_template_transferred_with_model)),
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
for crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
            .do_update()
            .set(self)
            .filter(
                liters
                    .ne(excluded(liters))
                    .or(stratified_source_model.ne(excluded(stratified_source_model)))
                    .or(
                        foreign_procedure_template
                            .ne(excluded(foreign_procedure_template)),
                    )
                    .or(
                        procedure_template_stratified_source_model
                            .ne(excluded(procedure_template_stratified_source_model)),
                    )
                    .or(
                        supernatant_destination_model
                            .ne(excluded(supernatant_destination_model)),
                    )
                    .or(
                        procedure_template_supernatant_destination_model
                            .ne(
                                excluded(procedure_template_supernatant_destination_model),
                            ),
                    )
                    .or(transferred_with_model.ne(excluded(transferred_with_model)))
                    .or(
                        procedure_template_transferred_with_model
                            .ne(excluded(procedure_template_transferred_with_model)),
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
