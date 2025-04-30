#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::srid,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_srid
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_srid,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::srtext
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::srtext,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::proj4text
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::proj4text,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
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
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::srid,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_name
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_name,
                                    ),
                                ),
                            crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_srid
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::auth_srid,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::srtext
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::srtext,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::proj4text
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys::proj4text,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
