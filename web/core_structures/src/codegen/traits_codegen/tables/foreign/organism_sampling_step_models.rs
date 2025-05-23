#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
>
for crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        diesel::result::Error,
    > {
        self.id(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::organisms::Organism,
>
for crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organisms::Organism,
        diesel::result::Error,
    > {
        self.organism(conn).await
    }
}
