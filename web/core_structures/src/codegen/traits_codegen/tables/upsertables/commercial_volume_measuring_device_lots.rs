#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_lots::commercial_volume_measuring_device_lots::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(product_model.ne(excluded(product_model)))
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_lots::commercial_volume_measuring_device_lots::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(product_model.ne(excluded(product_model)))
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
