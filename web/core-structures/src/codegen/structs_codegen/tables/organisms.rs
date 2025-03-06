#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: organisms :: organisms))]
pub struct Organism {
    pub id: uuid::Uuid,
    pub host_organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub notes: Option<String>,
    pub nameplate_id: i32,
    pub project_id: i32,
    pub created_by: i32,
    pub updated_by: i32,
    pub wild: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl Organism {
    #[cfg(feature = "postgres")]
    pub async fn host_organism(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::organisms::Organism>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(host_organism_id) = self.host_organism_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::organisms::Organism::table()
            .find(host_organism_id)
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
    pub async fn nameplate(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::nameplates::Nameplate, diesel::result::Error>
    {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::nameplates::Nameplate::table()
            .find(&self.nameplate_id)
            .first::<crate::codegen::structs_codegen::tables::nameplates::Nameplate>(conn)
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
}
