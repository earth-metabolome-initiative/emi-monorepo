#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                kilograms
                    .ne(excluded(kilograms))
                    .or(tolerance_percentage.ne(excluded(tolerance_percentage)))
                    .or(weighed_with_model.ne(excluded(weighed_with_model)))
                    .or(
                        procedure_template_weighed_with_model
                            .ne(excluded(procedure_template_weighed_with_model)),
                    )
                    .or(fragment_container_model.ne(excluded(fragment_container_model)))
                    .or(
                        procedure_template_fragment_container_model
                            .ne(excluded(procedure_template_fragment_container_model)),
                    )
                    .or(
                        fragment_placed_into_model
                            .ne(excluded(fragment_placed_into_model)),
                    )
                    .or(
                        procedure_template_fragment_placed_into_model
                            .ne(excluded(procedure_template_fragment_placed_into_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                kilograms
                    .ne(excluded(kilograms))
                    .or(tolerance_percentage.ne(excluded(tolerance_percentage)))
                    .or(weighed_with_model.ne(excluded(weighed_with_model)))
                    .or(
                        procedure_template_weighed_with_model
                            .ne(excluded(procedure_template_weighed_with_model)),
                    )
                    .or(fragment_container_model.ne(excluded(fragment_container_model)))
                    .or(
                        procedure_template_fragment_container_model
                            .ne(excluded(procedure_template_fragment_container_model)),
                    )
                    .or(
                        fragment_placed_into_model
                            .ne(excluded(fragment_placed_into_model)),
                    )
                    .or(
                        procedure_template_fragment_placed_into_model
                            .ne(excluded(procedure_template_fragment_placed_into_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
