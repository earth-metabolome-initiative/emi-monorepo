#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::instruments::instruments)]
pub struct Instrument {
    pub id: i32,
    pub status: String,
    pub user_created: Option<rosetta_uuid::Uuid>,
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    pub user_updated: Option<rosetta_uuid::Uuid>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub uuid_instrument: rosetta_uuid::Uuid,
    pub instrument_id: String,
    pub instrument_model: i32,
    pub instrument_location: Option<i32>,
}
impl Instrument {
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
    pub async fn instrument_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instrument_models::instrument_models::dsl::id
                    .eq(&self.instrument_model),
            )
            .first::<
                crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn instrument_location(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::rooms::Room>, diesel::result::Error>
    {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let Some(instrument_location) = self.instrument_location.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::rooms::Room::table()
            .filter(
                crate::codegen::diesel_codegen::tables::rooms::rooms::dsl::id
                    .eq(instrument_location),
            )
            .first::<crate::codegen::structs_codegen::tables::rooms::Room>(conn)
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
                crate::codegen::diesel_codegen::tables::instruments::instruments::dsl::user_created
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
                crate::codegen::diesel_codegen::tables::instruments::instruments::dsl::user_updated
                    .eq(user_updated.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_instrument_model(
        conn: &mut diesel_async::AsyncPgConnection,
        instrument_model: &crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instruments::instruments::dsl::instrument_model
                    .eq(instrument_model.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_instrument_location(
        conn: &mut diesel_async::AsyncPgConnection,
        instrument_location: &crate::codegen::structs_codegen::tables::rooms::Room,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instruments::instruments::dsl::instrument_location
                    .eq(instrument_location.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_instrument_id(
        instrument_id: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::instruments::instruments::instrument_id,
                instrument_id,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
