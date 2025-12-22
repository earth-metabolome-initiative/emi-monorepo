#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::supernatant_procedures::supernatant_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(stratified_source.ne(excluded(stratified_source)))
                    .or(procedure_template_stratified_source_model
                        .ne(excluded(procedure_template_stratified_source_model)))
                    .or(procedure_stratified_source.ne(excluded(procedure_stratified_source)))
                    .or(supernatant_destination.ne(excluded(supernatant_destination)))
                    .or(procedure_template_supernatant_destination_model
                        .ne(excluded(procedure_template_supernatant_destination_model)))
                    .or(procedure_supernatant_destination
                        .ne(excluded(procedure_supernatant_destination)))
                    .or(transferred_with.ne(excluded(transferred_with)))
                    .or(transferred_with_model.ne(excluded(transferred_with_model)))
                    .or(procedure_template_transferred_with_model
                        .ne(excluded(procedure_template_transferred_with_model)))
                    .or(procedure_transferred_with.ne(excluded(procedure_transferred_with)))
                    .or(pipette_tip_model.ne(excluded(pipette_tip_model)))
                    .or(procedure_template_pipette_tip_model
                        .ne(excluded(procedure_template_pipette_tip_model)))
                    .or(procedure_pipette_tip.ne(excluded(procedure_pipette_tip))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::supernatant_procedures::supernatant_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(stratified_source.ne(excluded(stratified_source)))
                    .or(procedure_template_stratified_source_model
                        .ne(excluded(procedure_template_stratified_source_model)))
                    .or(procedure_stratified_source.ne(excluded(procedure_stratified_source)))
                    .or(supernatant_destination.ne(excluded(supernatant_destination)))
                    .or(procedure_template_supernatant_destination_model
                        .ne(excluded(procedure_template_supernatant_destination_model)))
                    .or(procedure_supernatant_destination
                        .ne(excluded(procedure_supernatant_destination)))
                    .or(transferred_with.ne(excluded(transferred_with)))
                    .or(transferred_with_model.ne(excluded(transferred_with_model)))
                    .or(procedure_template_transferred_with_model
                        .ne(excluded(procedure_template_transferred_with_model)))
                    .or(procedure_transferred_with.ne(excluded(procedure_transferred_with)))
                    .or(pipette_tip_model.ne(excluded(pipette_tip_model)))
                    .or(procedure_template_pipette_tip_model
                        .ne(excluded(procedure_template_pipette_tip_model)))
                    .or(procedure_pipette_tip.ne(excluded(procedure_pipette_tip))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
