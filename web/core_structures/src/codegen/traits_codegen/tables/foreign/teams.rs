#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::colors::Color>
    for crate::codegen::structs_codegen::tables::teams::Team
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        self.color(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::team_states::TeamState,
    > for crate::codegen::structs_codegen::tables::teams::Team
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::team_states::TeamState,
        diesel::result::Error,
    > {
        self.state(conn).await
    }
}
