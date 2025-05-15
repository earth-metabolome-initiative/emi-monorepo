#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::trackables::trackables)]
pub struct Trackable {
    pub id: rosetta_uuid::Uuid,
    pub container_model_id: i32,
    pub project_id: i32,
    pub trackable_state_id: i16,
    pub created_by: i32,
    pub created_at: rosetta_timestamp::TimestampUTC,
    pub updated_by: i32,
    pub updated_at: rosetta_timestamp::TimestampUTC,
}
impl diesel::Identifiable for Trackable {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Trackable {
    #[cfg(feature = "postgres")]
    pub async fn container_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::container_models::ContainerModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::id
                    .eq(&self.container_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::container_models::ContainerModel>(
                conn,
            )
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn project(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::projects::Project, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::projects::Project::table()
            .filter(
                crate::codegen::diesel_codegen::tables::projects::projects::dsl::id
                    .eq(&self.project_id),
            )
            .first::<crate::codegen::structs_codegen::tables::projects::Project>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn trackable_state(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackable_states::TrackableState,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::trackable_states::TrackableState::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackable_states::trackable_states::dsl::id
                    .eq(&self.trackable_state_id),
            )
            .first::<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>(
                conn,
            )
            .await
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
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_container_model_id(
        conn: &mut diesel_async::AsyncPgConnection,
        container_model_id: &crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::container_model_id
                    .eq(container_model_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_project_id(
        conn: &mut diesel_async::AsyncPgConnection,
        project_id: &crate::codegen::structs_codegen::tables::projects::Project,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::project_id
                    .eq(project_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_trackable_state_id(
        conn: &mut diesel_async::AsyncPgConnection,
        trackable_state_id: &crate::codegen::structs_codegen::tables::trackable_states::TrackableState,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::trackable_state_id
                    .eq(trackable_state_id.id),
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
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::created_by
                    .eq(created_by.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_updated_by(
        conn: &mut diesel_async::AsyncPgConnection,
        updated_by: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::updated_by
                    .eq(updated_by.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_created_at(
        created_at: &rosetta_timestamp::TimestampUTC,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        Self::table()
            .filter(trackables::created_at.eq(created_at))
            .order_by(trackables::id.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_updated_at(
        updated_at: &rosetta_timestamp::TimestampUTC,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        Self::table()
            .filter(trackables::updated_at.eq(updated_at))
            .order_by(trackables::id.asc())
            .load::<Self>(conn)
            .await
    }
}
