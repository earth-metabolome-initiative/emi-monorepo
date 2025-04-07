#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::projects::projects)]
pub struct Project {
    pub id: i32,
    pub status: Option<String>,
    pub user_created: Option<uuid::Uuid>,
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    pub user_updated: Option<uuid::Uuid>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub uuid_project: Option<uuid::Uuid>,
    pub project_id: String,
    pub project_description: String,
    pub parent_project: Option<i32>,
    pub batch: i32,
}
impl Project {
    #[cfg(feature = "postgres")]
    pub async fn user_created(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(user_created) = self.user_created.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user_created),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user_created(
        conn: &mut diesel_async::AsyncPgConnection,
        user_created: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::user_created
                    .eq(&user_created.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn user_updated(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(user_updated) = self.user_updated.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user_updated),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user_updated(
        conn: &mut diesel_async::AsyncPgConnection,
        user_updated: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::user_updated
                    .eq(&user_updated.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn parent_project(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::projects::Project>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(parent_project) = self.parent_project.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::projects::Project::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::id
                    .eq(parent_project),
            )
            .first::<crate::codegen::structs_codegen::tables::projects::Project>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent_project(
        conn: &mut diesel_async::AsyncPgConnection,
        parent_project: &crate::codegen::structs_codegen::tables::projects::Project,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::parent_project
                    .eq(parent_project.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn batch(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::batches::Batch,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::batches::Batch::table()
            .filter(
                crate::codegen::diesel_codegen::tables::batches::batches::dsl::id
                    .eq(&self.batch),
            )
            .first::<crate::codegen::structs_codegen::tables::batches::Batch>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_batch(
        conn: &mut diesel_async::AsyncPgConnection,
        batch: &crate::codegen::structs_codegen::tables::batches::Batch,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::batch
                    .eq(batch.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_project_id(
        project_id: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::projects::projects::project_id,
                    project_id,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
