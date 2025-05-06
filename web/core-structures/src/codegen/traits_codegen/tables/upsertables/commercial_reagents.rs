#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::id,
            )
            .do_update()
            .set(self)
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::commercial_product_lot_id
                    .ne(
                        diesel::upsert::excluded(
                            crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::commercial_product_lot_id,
                        ),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::id,
            )
            .do_update()
            .set(self)
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::commercial_product_lot_id
                    .ne(
                        diesel::upsert::excluded(
                            crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::commercial_product_lot_id,
                        ),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
