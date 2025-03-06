#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: sample_containers :: sample_containers))]
pub struct SampleContainer {
    pub id: i32,
    pub barcode: String,
    pub project_id: i32,
    pub created_by: i32,
    pub updated_by: i32,
    pub category_id: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl SampleContainer {
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
    #[cfg(feature = "postgres")]    pub async fn category (& self , conn : & mut diesel_async :: AsyncPgConnection) -> Result < crate :: codegen :: structs_codegen :: tables :: sample_container_categories :: SampleContainerCategory , diesel :: result :: Error >{
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate :: codegen :: structs_codegen :: tables :: sample_container_categories :: SampleContainerCategory :: table () . find (& self . category_id) . first :: < crate :: codegen :: structs_codegen :: tables :: sample_container_categories :: SampleContainerCategory > (conn) . await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_barcode(
        barcode: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self :: table () . filter (diesel :: ExpressionMethods :: eq (crate :: codegen :: diesel_codegen :: tables :: sample_containers :: sample_containers :: barcode , barcode)) . first :: < Self > (conn) . await . optional ()
    }
}
