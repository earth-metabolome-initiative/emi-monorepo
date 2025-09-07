#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection> for crate::CentrifugeProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(centrifuged_container.ne(excluded(centrifuged_container)))
                    .or(centrifuged_container_model.ne(excluded(centrifuged_container_model)))
                    .or(procedure_template_centrifuged_container_model
                        .ne(excluded(procedure_template_centrifuged_container_model)))
                    .or(procedure_centrifuged_container
                        .ne(excluded(procedure_centrifuged_container)))
                    .or(centrifuged_with_model.ne(excluded(centrifuged_with_model)))
                    .or(centrifuged_with.ne(excluded(centrifuged_with)))
                    .or(procedure_template_centrifuged_with_model
                        .ne(excluded(procedure_template_centrifuged_with_model)))
                    .or(procedure_centrifuged_with.ne(excluded(procedure_centrifuged_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::CentrifugeProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(centrifuged_container.ne(excluded(centrifuged_container)))
                    .or(centrifuged_container_model.ne(excluded(centrifuged_container_model)))
                    .or(procedure_template_centrifuged_container_model
                        .ne(excluded(procedure_template_centrifuged_container_model)))
                    .or(procedure_centrifuged_container
                        .ne(excluded(procedure_centrifuged_container)))
                    .or(centrifuged_with_model.ne(excluded(centrifuged_with_model)))
                    .or(centrifuged_with.ne(excluded(centrifuged_with)))
                    .or(procedure_template_centrifuged_with_model
                        .ne(excluded(procedure_template_centrifuged_with_model)))
                    .or(procedure_centrifuged_with.ne(excluded(procedure_centrifuged_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
