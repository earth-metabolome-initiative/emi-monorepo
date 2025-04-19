#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
>
for crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    > {
        self.instrument_model(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
>
for crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
        diesel::result::Error,
    > {
        self.instrument_category(conn).await
    }
}
