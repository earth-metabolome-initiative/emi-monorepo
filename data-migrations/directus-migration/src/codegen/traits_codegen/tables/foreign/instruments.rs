#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    > for crate::codegen::structs_codegen::tables::instruments::Instrument
{
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
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::rooms::Room>
    for crate::codegen::structs_codegen::tables::instruments::Instrument
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::rooms::Room, diesel::result::Error> {
        self.instrument_location(conn).await
    }
}
