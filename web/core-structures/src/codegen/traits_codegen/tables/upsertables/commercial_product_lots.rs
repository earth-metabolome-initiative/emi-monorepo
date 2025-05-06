#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::lot
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::lot,
                            ),
                        ),
                    crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::product_model_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::product_model_id,
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
    for crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl};
        diesel::insert_into(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::table,
            )
            .values(self)
            .on_conflict(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::id,
            )
            .do_update()
            .set(self)
            .filter(
                diesel::BoolExpressionMethods::and(
                    crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::lot
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::lot,
                            ),
                        ),
                    crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::product_model_id
                        .ne(
                            diesel::upsert::excluded(
                                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::product_model_id,
                            ),
                        ),
                ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
