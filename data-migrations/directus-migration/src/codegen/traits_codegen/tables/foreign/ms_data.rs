#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::si_units::SiUnit>
    for crate::codegen::structs_codegen::tables::ms_data::MsDatum
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::si_units::SiUnit, diesel::result::Error>
    {
        self.injection_volume_unit(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod,
    > for crate::codegen::structs_codegen::tables::ms_data::MsDatum
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod,
        diesel::result::Error,
    > {
        self.injection_method(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
    > for crate::codegen::structs_codegen::tables::ms_data::MsDatum
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
        diesel::result::Error,
    > {
        self.instrument_used(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::containers::Container,
    > for crate::codegen::structs_codegen::tables::ms_data::MsDatum
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    {
        self.parent_sample_container(conn).await
    }
}
