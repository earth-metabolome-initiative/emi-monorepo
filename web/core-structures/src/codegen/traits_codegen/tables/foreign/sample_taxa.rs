#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::users::User>
    for crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        self.created_by(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::samples::Sample>
    for crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::samples::Sample, diesel::result::Error>
    {
        self.sample(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::taxa::Taxa>
    for crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::taxa::Taxa, diesel::result::Error> {
        self.taxon(conn).await
    }
}
