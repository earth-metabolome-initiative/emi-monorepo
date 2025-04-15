#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::container_models::container_models
)]
pub struct ContainerModel {
    pub id: i32,
    pub status: String,
    pub user_created: Option<rosetta_uuid::Uuid>,
    pub date_created: Option<rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<rosetta_uuid::Uuid>,
    pub date_updated: Option<rosetta_timestamp::TimestampUTC>,
    pub container_type: i32,
    pub volume: f32,
    pub volume_unit: i32,
    pub brand: i32,
    pub is_sample_container: bool,
    pub __columns: i32,
    pub columns_numeric: bool,
    pub rows: i32,
    pub rows_numeric: bool,
}
impl ContainerModel {
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
    pub async fn container_type(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_types::ContainerType,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::container_types::ContainerType::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_types::container_types::dsl::id
                    .eq(&self.container_type),
            )
            .first::<crate::codegen::structs_codegen::tables::container_types::ContainerType>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn volume_unit(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::si_units::SiUnit, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::si_units::SiUnit::table()
            .filter(
                crate::codegen::diesel_codegen::tables::si_units::si_units::dsl::id
                    .eq(&self.volume_unit),
            )
            .first::<crate::codegen::structs_codegen::tables::si_units::SiUnit>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn brand(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::brands::Brand, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::brands::Brand::table()
            .filter(crate::codegen::diesel_codegen::tables::brands::brands::dsl::id.eq(&self.brand))
            .first::<crate::codegen::structs_codegen::tables::brands::Brand>(conn)
            .await
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
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::user_created
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
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::user_updated
                    .eq(user_updated.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_container_type(
        conn: &mut diesel_async::AsyncPgConnection,
        container_type: &crate::codegen::structs_codegen::tables::container_types::ContainerType,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::container_type
                    .eq(container_type.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_volume_unit(
        conn: &mut diesel_async::AsyncPgConnection,
        volume_unit: &crate::codegen::structs_codegen::tables::si_units::SiUnit,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::volume_unit
                    .eq(volume_unit.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_brand(
        conn: &mut diesel_async::AsyncPgConnection,
        brand: &crate::codegen::structs_codegen::tables::brands::Brand,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::brand
                    .eq(brand.id),
            )
            .load::<Self>(conn)
            .await
    }
}
