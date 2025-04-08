#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data
)]
pub struct DriedSamplesDatum {
    pub id: i32,
    pub status: String,
    pub user_created: Option<rosetta_uuid::Uuid>,
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    pub user_updated: Option<rosetta_uuid::Uuid>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub uuid_dried_sample: Option<rosetta_uuid::Uuid>,
    pub sample_container: i32,
    pub parent_container: Option<i32>,
    pub batch: Option<i32>,
    pub field_data: Option<i32>,
}
impl DriedSamplesDatum {
    #[cfg(feature = "postgres")]
    pub async fn user_created(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(user_created) = self.user_created.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user_created),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn user_updated(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(user_updated) = self.user_updated.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user_updated),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn sample_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::containers::Container::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::id
                    .eq(&self.sample_container),
            )
            .first::<crate::codegen::structs_codegen::tables::containers::Container>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn parent_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::containers::Container>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(parent_container) = self.parent_container.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::containers::Container::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::id
                    .eq(parent_container),
            )
            .first::<crate::codegen::structs_codegen::tables::containers::Container>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn batch(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::batches::Batch>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(batch) = self.batch.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::batches::Batch::table()
            .filter(crate::codegen::diesel_codegen::tables::batches::batches::dsl::id.eq(batch))
            .first::<crate::codegen::structs_codegen::tables::batches::Batch>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn field_data(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::field_data::FieldDatum>,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(field_data) = self.field_data.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::field_data::FieldDatum::table()
            .filter(
                crate::codegen::diesel_codegen::tables::field_data::field_data::dsl::id
                    .eq(field_data),
            )
            .first::<crate::codegen::structs_codegen::tables::field_data::FieldDatum>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user_created(
        conn: &mut diesel_async::AsyncPgConnection,
        user_created: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data::dsl::user_created
                    .eq(user_created.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user_updated(
        conn: &mut diesel_async::AsyncPgConnection,
        user_updated: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data::dsl::user_updated
                    .eq(user_updated.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_sample_container(
        conn: &mut diesel_async::AsyncPgConnection,
        sample_container: &crate::codegen::structs_codegen::tables::containers::Container,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data::dsl::sample_container
                    .eq(sample_container.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent_container(
        conn: &mut diesel_async::AsyncPgConnection,
        parent_container: &crate::codegen::structs_codegen::tables::containers::Container,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data::dsl::parent_container
                    .eq(parent_container.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_batch(
        conn: &mut diesel_async::AsyncPgConnection,
        batch: &crate::codegen::structs_codegen::tables::batches::Batch,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data::dsl::batch
                    .eq(batch.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_field_data(
        conn: &mut diesel_async::AsyncPgConnection,
        field_data: &crate::codegen::structs_codegen::tables::field_data::FieldDatum,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data::dsl::field_data
                    .eq(field_data.id),
            )
            .load::<Self>(conn)
            .await
    }
}
