#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::table,
        )
        .values(self)
        .on_conflict((
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::organism_id,
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::taxon_id,
        ))
        .do_update()
        .set(self)
        .filter(diesel::BoolExpressionMethods::and(
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_by
                .ne(diesel::upsert::excluded(
                crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_by,
            )),
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_at
                .ne(diesel::upsert::excluded(
                crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_at,
            )),
        ))
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
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::table,
        )
        .values(self)
        .on_conflict((
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::organism_id,
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::taxon_id,
        ))
        .do_update()
        .set(self)
        .filter(diesel::BoolExpressionMethods::and(
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_by
                .ne(diesel::upsert::excluded(
                crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_by,
            )),
            crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_at
                .ne(diesel::upsert::excluded(
                crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_at,
            )),
        ))
        .get_results(conn)
        .map(|mut result| result.pop())
    }
}
