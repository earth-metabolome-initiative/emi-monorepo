#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
            .do_update()
            .set(self)
            .filter(
                photographed_with_model
                    .ne(excluded(photographed_with_model))
                    .or(
                        procedure_template_photographed_with_model
                            .ne(excluded(procedure_template_photographed_with_model)),
                    )
                    .or(photographed_asset_model.ne(excluded(photographed_asset_model)))
                    .or(
                        procedure_template_photographed_asset_model
                            .ne(excluded(procedure_template_photographed_asset_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
            .do_update()
            .set(self)
            .filter(
                photographed_with_model
                    .ne(excluded(photographed_with_model))
                    .or(
                        procedure_template_photographed_with_model
                            .ne(excluded(procedure_template_photographed_with_model)),
                    )
                    .or(photographed_asset_model.ne(excluded(photographed_asset_model)))
                    .or(
                        procedure_template_photographed_asset_model
                            .ne(excluded(procedure_template_photographed_asset_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
