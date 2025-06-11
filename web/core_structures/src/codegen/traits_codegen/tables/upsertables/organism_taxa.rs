#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((organism_id, taxon_id))
            .do_update()
            .set(self)
            .filter(created_by.ne(excluded(created_by)).or(created_at.ne(excluded(created_at))))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((organism_id, taxon_id))
            .do_update()
            .set(self)
            .filter(created_by.ne(excluded(created_by)).or(created_at.ne(excluded(created_at))))
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
