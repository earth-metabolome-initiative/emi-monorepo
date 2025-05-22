#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<crate::codegen::structs_codegen::tables::projects::Project>
    for crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation
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
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::organisms::Organism,
    > for crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<crate::codegen::structs_codegen::tables::organisms::Organism, diesel::result::Error>
    {
        self.organism(conn).await
    }
}
#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    > for crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        diesel::result::Error,
    > {
        self.subject(conn).await
    }
}
