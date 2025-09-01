#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::shared_procedure_template_asset_models::shared_procedure_template_asset_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((parent, child_id))
            .do_update()
            .set(self)
            .filter(
                parent_asset_model
                    .ne(excluded(parent_asset_model))
                    .or(
                        parent_procedure_template.ne(excluded(parent_procedure_template)),
                    )
                    .or(child_asset_model.ne(excluded(child_asset_model)))
                    .or(child_procedure_template.ne(excluded(child_procedure_template)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::shared_procedure_template_asset_models::shared_procedure_template_asset_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((parent, child_id))
            .do_update()
            .set(self)
            .filter(
                parent_asset_model
                    .ne(excluded(parent_asset_model))
                    .or(
                        parent_procedure_template.ne(excluded(parent_procedure_template)),
                    )
                    .or(child_asset_model.ne(excluded(child_asset_model)))
                    .or(child_procedure_template.ne(excluded(child_procedure_template)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
