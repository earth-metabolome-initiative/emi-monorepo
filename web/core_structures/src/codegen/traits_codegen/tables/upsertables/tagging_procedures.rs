#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::tagging_procedures::tagging_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(tagged_asset.ne(excluded(tagged_asset)))
                    .or(procedure_template_tagged_asset_model
                        .ne(excluded(procedure_template_tagged_asset_model)))
                    .or(procedure_tagged_asset.ne(excluded(procedure_tagged_asset)))
                    .or(tag_asset.ne(excluded(tag_asset)))
                    .or(procedure_template_tag_asset_model
                        .ne(excluded(procedure_template_tag_asset_model)))
                    .or(procedure_tag_asset.ne(excluded(procedure_tag_asset)))
                    .or(geolocated_with.ne(excluded(geolocated_with)))
                    .or(procedure_geolocated_with.ne(excluded(procedure_geolocated_with)))
                    .or(procedure_template_geolocated_with_model
                        .ne(excluded(procedure_template_geolocated_with_model)))
                    .or(location.ne(excluded(location))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::tagging_procedures::tagging_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(tagged_asset.ne(excluded(tagged_asset)))
                    .or(procedure_template_tagged_asset_model
                        .ne(excluded(procedure_template_tagged_asset_model)))
                    .or(procedure_tagged_asset.ne(excluded(procedure_tagged_asset)))
                    .or(tag_asset.ne(excluded(tag_asset)))
                    .or(procedure_template_tag_asset_model
                        .ne(excluded(procedure_template_tag_asset_model)))
                    .or(procedure_tag_asset.ne(excluded(procedure_tag_asset)))
                    .or(geolocated_with.ne(excluded(geolocated_with)))
                    .or(procedure_geolocated_with.ne(excluded(procedure_geolocated_with)))
                    .or(procedure_template_geolocated_with_model
                        .ne(excluded(procedure_template_geolocated_with_model)))
                    .or(location.ne(excluded(location))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
