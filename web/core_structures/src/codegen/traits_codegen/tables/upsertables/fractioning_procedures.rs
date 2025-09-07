#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection> for crate::FractioningProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(fragment_container.ne(excluded(fragment_container)))
                    .or(procedure_template_fragment_container_model
                        .ne(excluded(procedure_template_fragment_container_model)))
                    .or(procedure_fragment_container.ne(excluded(procedure_fragment_container)))
                    .or(fragment_placed_into.ne(excluded(fragment_placed_into)))
                    .or(procedure_template_fragment_placed_into_model
                        .ne(excluded(procedure_template_fragment_placed_into_model)))
                    .or(procedure_fragment_placed_into.ne(excluded(procedure_fragment_placed_into)))
                    .or(kilograms.ne(excluded(kilograms)))
                    .or(weighed_with.ne(excluded(weighed_with)))
                    .or(procedure_template_weighed_with_model
                        .ne(excluded(procedure_template_weighed_with_model)))
                    .or(procedure_weighed_with.ne(excluded(procedure_weighed_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::FractioningProcedure
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
        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(fragment_container.ne(excluded(fragment_container)))
                    .or(procedure_template_fragment_container_model
                        .ne(excluded(procedure_template_fragment_container_model)))
                    .or(procedure_fragment_container.ne(excluded(procedure_fragment_container)))
                    .or(fragment_placed_into.ne(excluded(fragment_placed_into)))
                    .or(procedure_template_fragment_placed_into_model
                        .ne(excluded(procedure_template_fragment_placed_into_model)))
                    .or(procedure_fragment_placed_into.ne(excluded(procedure_fragment_placed_into)))
                    .or(kilograms.ne(excluded(kilograms)))
                    .or(weighed_with.ne(excluded(weighed_with)))
                    .or(procedure_template_weighed_with_model
                        .ne(excluded(procedure_template_weighed_with_model)))
                    .or(procedure_weighed_with.ne(excluded(procedure_weighed_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
