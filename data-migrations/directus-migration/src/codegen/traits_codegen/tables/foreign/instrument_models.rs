#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::instrument_types::InstrumentType,
> for crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_types::InstrumentType,
        diesel::result::Error,
    > {
        self.instrument_type(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::brands::Brand,
> for crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::brands::Brand,
        diesel::result::Error,
    > {
        self.brand(conn).await
    }
}
