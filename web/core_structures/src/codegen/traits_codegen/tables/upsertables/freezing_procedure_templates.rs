#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::freezing_procedure_templates::freezing_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
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
                    .or(frozen_with_model.ne(excluded(frozen_with_model)))
                    .or(
                        procedure_template_frozen_with_model
                            .ne(excluded(procedure_template_frozen_with_model)),
                    )
                    .or(frozen_container_model.ne(excluded(frozen_container_model)))
                    .or(
                        foreign_procedure_template
                            .ne(excluded(foreign_procedure_template)),
                    )
                    .or(
                        procedure_template_frozen_container_model
                            .ne(excluded(procedure_template_frozen_container_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::freezing_procedure_templates::freezing_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
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
                    .or(frozen_with_model.ne(excluded(frozen_with_model)))
                    .or(
                        procedure_template_frozen_with_model
                            .ne(excluded(procedure_template_frozen_with_model)),
                    )
                    .or(frozen_container_model.ne(excluded(frozen_container_model)))
                    .or(
                        foreign_procedure_template
                            .ne(excluded(foreign_procedure_template)),
                    )
                    .or(
                        procedure_template_frozen_container_model
                            .ne(excluded(procedure_template_frozen_container_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
