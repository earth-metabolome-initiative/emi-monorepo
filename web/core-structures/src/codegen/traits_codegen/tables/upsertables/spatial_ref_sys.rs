#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(srid)
            .do_update()
            .set(self)
            .filter(
                auth_name
                    .ne(excluded(auth_name))
                    .or(auth_srid.ne(excluded(auth_srid)))
                    .or(srtext.ne(excluded(srtext)))
                    .or(proj4text.ne(excluded(proj4text))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(srid)
            .do_update()
            .set(self)
            .filter(
                auth_name
                    .ne(excluded(auth_name))
                    .or(auth_srid.ne(excluded(auth_srid)))
                    .or(srtext.ne(excluded(srtext)))
                    .or(proj4text.ne(excluded(proj4text))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
