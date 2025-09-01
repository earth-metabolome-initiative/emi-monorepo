#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure
{
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
                    .or(foreign_procedure_template.ne(excluded(foreign_procedure_template)))
                    .or(foreign_procedure.ne(excluded(foreign_procedure)))
                    .or(weighed_container.ne(excluded(weighed_container)))
                    .or(kilograms.ne(excluded(kilograms)))
                    .or(weighed_with_model.ne(excluded(weighed_with_model)))
                    .or(weighed_with.ne(excluded(weighed_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure
{
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
                    .or(foreign_procedure_template.ne(excluded(foreign_procedure_template)))
                    .or(foreign_procedure.ne(excluded(foreign_procedure)))
                    .or(weighed_container.ne(excluded(weighed_container)))
                    .or(kilograms.ne(excluded(kilograms)))
                    .or(weighed_with_model.ne(excluded(weighed_with_model)))
                    .or(weighed_with.ne(excluded(weighed_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
