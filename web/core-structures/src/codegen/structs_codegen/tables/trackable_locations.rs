#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations
)]
pub struct TrackableLocation {
    pub id: rosetta_uuid::Uuid,
    pub trackable_id: rosetta_uuid::Uuid,
    pub storage_container_id: Option<rosetta_uuid::Uuid>,
    pub geolocation: postgis_diesel::types::Point,
    pub inferred: bool,
    pub created_at: rosetta_timestamp::TimestampUTC,
    pub created_by: i32,
}
impl diesel::Identifiable for TrackableLocation {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl TrackableLocation {
    #[cfg(feature = "postgres")]
    pub async fn trackable(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::trackables::Trackable, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::trackables::Trackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::id
                    .eq(&self.trackable_id),
            )
            .first::<crate::codegen::structs_codegen::tables::trackables::Trackable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn storage_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(storage_container_id) = self.storage_container_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::storage_containers::StorageContainer::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_containers::storage_containers::dsl::id
                    .eq(storage_container_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_trackable_id(
        conn: &mut diesel_async::AsyncPgConnection,
        trackable_id: &crate::codegen::structs_codegen::tables::trackables::Trackable,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::dsl::trackable_id
                    .eq(trackable_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_storage_container_id(
        conn: &mut diesel_async::AsyncPgConnection,
        storage_container_id: &crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::dsl::storage_container_id
                    .eq(storage_container_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_created_by(
        conn: &mut diesel_async::AsyncPgConnection,
        created_by: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackable_locations::trackable_locations::dsl::created_by
                    .eq(created_by.id),
            )
            .load::<Self>(conn)
            .await
    }
}
