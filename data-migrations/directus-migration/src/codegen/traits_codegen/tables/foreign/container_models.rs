#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::container_types::ContainerType,
    > for crate::codegen::structs_codegen::tables::container_models::ContainerModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_types::ContainerType,
        diesel::result::Error,
    > {
        self.container_type(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::si_units::SiUnit>
    for crate::codegen::structs_codegen::tables::container_models::ContainerModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::si_units::SiUnit, diesel::result::Error>
    {
        self.volume_unit(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::brands::Brand>
    for crate::codegen::structs_codegen::tables::container_models::ContainerModel
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::brands::Brand, diesel::result::Error> {
        self.brand(conn).await
    }
}
