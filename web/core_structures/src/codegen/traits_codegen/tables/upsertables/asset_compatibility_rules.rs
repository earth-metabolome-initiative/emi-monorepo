#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::AssetCompatibilityRule
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
        use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((left_asset_model, right_asset_model))
            .do_update()
            .set(self)
            .filter(created_by.ne(excluded(created_by)).or(created_at.ne(excluded(created_at))))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::AssetCompatibilityRule
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
        use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((left_asset_model, right_asset_model))
            .do_update()
            .set(self)
            .filter(created_by.ne(excluded(created_by)).or(created_at.ne(excluded(created_at))))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
