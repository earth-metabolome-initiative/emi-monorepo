#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection> for crate::WeighingProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::weighing_procedures::weighing_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(weighed_container.ne(excluded(weighed_container)))
                    .or(procedure_template_weighed_container_model
                        .ne(excluded(procedure_template_weighed_container_model)))
                    .or(procedure_weighed_container.ne(excluded(procedure_weighed_container)))
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
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection> for crate::WeighingProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::weighing_procedures::weighing_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(weighed_container.ne(excluded(weighed_container)))
                    .or(procedure_template_weighed_container_model
                        .ne(excluded(procedure_template_weighed_container_model)))
                    .or(procedure_weighed_container.ne(excluded(procedure_weighed_container)))
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
