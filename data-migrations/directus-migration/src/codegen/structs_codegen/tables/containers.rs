#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::containers::containers)]
pub struct Container {
    pub id: i32,
    pub status: Option<String>,
    pub user_created: Option<rosetta_uuid::Uuid>,
    pub date_created: Option<rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<rosetta_uuid::Uuid>,
    pub date_updated: Option<rosetta_timestamp::TimestampUTC>,
    pub used: bool,
    pub reserved: bool,
    pub uuid_container: Option<rosetta_uuid::Uuid>,
    pub container_id: String,
    pub container_model: Option<i32>,
    pub is_finite: Option<bool>,
    pub __columns: Option<i32>,
    pub rows: Option<i32>,
    pub rows_numeric: Option<bool>,
    pub columns_numeric: Option<bool>,
    pub location: Option<i32>,
    pub old_id: Option<String>,
    pub parent_container: Option<i32>,
}
impl diesel::Identifiable for Container {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Container {
    #[cfg(feature = "postgres")]
    pub async fn user_created(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
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
    pub async fn container_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(container_model) = self.container_model.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::container_models::ContainerModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::id
                    .eq(container_model),
            )
            .first::<crate::codegen::structs_codegen::tables::container_models::ContainerModel>(
                conn,
            )
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn location(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::universities::University>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(location) = self.location.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::universities::University::table()
            .filter(
                crate::codegen::diesel_codegen::tables::universities::universities::dsl::id
                    .eq(location),
            )
            .first::<crate::codegen::structs_codegen::tables::universities::University>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn parent_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::containers::Container>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
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
    pub async fn from_user_created(
        conn: &mut diesel_async::AsyncPgConnection,
        user_created: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::user_created
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
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::user_updated
                    .eq(user_updated.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_container_model(
        conn: &mut diesel_async::AsyncPgConnection,
        container_model: &crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::container_model
                    .eq(container_model.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_location(
        conn: &mut diesel_async::AsyncPgConnection,
        location: &crate::codegen::structs_codegen::tables::universities::University,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::location
                    .eq(location.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent_container(
        conn: &mut diesel_async::AsyncPgConnection,
        parent_container: &crate::codegen::structs_codegen::tables::containers::Container,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::parent_container
                    .eq(parent_container.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_container_id(
        container_id: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::containers::containers::container_id,
                container_id,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_old_id(
        old_id: Option<&str>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::containers::containers::old_id,
                old_id,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
