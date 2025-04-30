#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::trackable_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::trackable_id,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::storage_container_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::storage_container_id,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::geolocation
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::geolocation,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::inferred
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::inferred,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_by
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_by,
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
    for crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::BoolExpressionMethods::and(
                            diesel::BoolExpressionMethods::and(
                                diesel::BoolExpressionMethods::and(
                                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::trackable_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::trackable_id,
                                            ),
                                        ),
                                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::storage_container_id
                                        .ne(
                                            diesel::upsert::excluded(
                                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::storage_container_id,
                                            ),
                                        ),
                                ),
                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::geolocation
                                    .ne(
                                        diesel::upsert::excluded(
                                            crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::geolocation,
                                        ),
                                    ),
                            ),
                            crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::inferred
                                .ne(
                                    diesel::upsert::excluded(
                                        crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::inferred,
                                    ),
                                ),
                        ),
                        crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_at
                            .ne(
                                diesel::upsert::excluded(
                                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_at,
                                ),
                            ),
                    ),
                    crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_by
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::created_by,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
