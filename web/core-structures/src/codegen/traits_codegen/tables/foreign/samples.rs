#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::sample_containers::SampleContainer,
    > for crate::codegen::structs_codegen::tables::samples::Sample
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_containers::SampleContainer,
        diesel::result::Error,
    > {
        self.container(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::projects::Project>
    for crate::codegen::structs_codegen::tables::samples::Sample
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::projects::Project, diesel::result::Error>
    {
        self.project(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::users::User>
    for crate::codegen::structs_codegen::tables::samples::Sample
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        self.updated_by(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::sample_states::SampleState,
    > for crate::codegen::structs_codegen::tables::samples::Sample
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_states::SampleState,
        diesel::result::Error,
    > {
        self.state(conn).await
    }
}
