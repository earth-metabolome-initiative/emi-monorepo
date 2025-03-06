#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: observations :: observations))]
pub struct Observation {
    pub id: uuid::Uuid,
    pub parent_observation_id: Option<uuid::Uuid>,
    pub created_by: i32,
    pub updated_by: i32,
    pub project_id: i32,
    pub organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub subject_id: i16,
    pub picture: Vec<u8>,
    pub notes: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl Observation {
    #[cfg(feature = "postgres")]
    pub async fn parent_observation(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::observations::Observation>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(parent_observation_id) = self.parent_observation_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::observations::Observation::table()
            .find(parent_observation_id)
            .first::<crate::codegen::structs_codegen::tables::observations::Observation>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .find(&self.created_by)
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .find(&self.updated_by)
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn project(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::projects::Project, diesel::result::Error>
    {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::projects::Project::table()
            .find(&self.project_id)
            .first::<crate::codegen::structs_codegen::tables::projects::Project>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn organism(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::organisms::Organism>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(organism_id) = self.organism_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::organisms::Organism::table()
            .find(organism_id)
            .first::<crate::codegen::structs_codegen::tables::organisms::Organism>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn sample(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::samples::Sample>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(sample_id) = self.sample_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::samples::Sample::table()
            .find(sample_id)
            .first::<crate::codegen::structs_codegen::tables::samples::Sample>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn subject(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate :: codegen :: structs_codegen :: tables :: observation_subjects :: ObservationSubject :: table () . find (& self . subject_id) . first :: < crate :: codegen :: structs_codegen :: tables :: observation_subjects :: ObservationSubject > (conn) . await
    }
}
