#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::steps::Step>
    for crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::steps::Step, diesel::result::Error> {
        self.id(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
    > for crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
        diesel::result::Error,
    > {
        self.instrument(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::users::User>
    for crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        self.created_by(conn).await
    }
}
