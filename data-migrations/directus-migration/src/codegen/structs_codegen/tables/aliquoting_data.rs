#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data
)]
pub struct AliquotingDatum {
    pub id: i32,
    pub status: String,
    pub user_created: Option<uuid::Uuid>,
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    pub user_updated: Option<uuid::Uuid>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub sample_container: i32,
    pub uuid_aliquot: Option<uuid::Uuid>,
    pub aliquot_volume: f32,
    pub aliquot_volume_unit: i32,
    pub parent_container: i32,
    pub parent_sample_container: i32,
}
impl AliquotingDatum {
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
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::dsl::user_created
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
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::dsl::user_updated
                    .eq(&user_updated.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn sample_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::containers::Container,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::containers::Container::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::id
                    .eq(&self.sample_container),
            )
            .first::<
                crate::codegen::structs_codegen::tables::containers::Container,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_sample_container(
        conn: &mut diesel_async::AsyncPgConnection,
        sample_container: &crate::codegen::structs_codegen::tables::containers::Container,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::dsl::sample_container
                    .eq(sample_container.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn aliquot_volume_unit(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::si_units::SiUnit,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::si_units::SiUnit::table()
            .filter(
                crate::codegen::diesel_codegen::tables::si_units::si_units::dsl::id
                    .eq(&self.aliquot_volume_unit),
            )
            .first::<crate::codegen::structs_codegen::tables::si_units::SiUnit>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_aliquot_volume_unit(
        conn: &mut diesel_async::AsyncPgConnection,
        aliquot_volume_unit: &crate::codegen::structs_codegen::tables::si_units::SiUnit,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::dsl::aliquot_volume_unit
                    .eq(aliquot_volume_unit.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn parent_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::containers::Container,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::containers::Container::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::id
                    .eq(&self.parent_container),
            )
            .first::<
                crate::codegen::structs_codegen::tables::containers::Container,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent_container(
        conn: &mut diesel_async::AsyncPgConnection,
        parent_container: &crate::codegen::structs_codegen::tables::containers::Container,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::dsl::parent_container
                    .eq(parent_container.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn parent_sample_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::containers::Container,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::containers::Container::table()
            .filter(
                crate::codegen::diesel_codegen::tables::containers::containers::dsl::id
                    .eq(&self.parent_sample_container),
            )
            .first::<
                crate::codegen::structs_codegen::tables::containers::Container,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent_sample_container(
        conn: &mut diesel_async::AsyncPgConnection,
        parent_sample_container: &crate::codegen::structs_codegen::tables::containers::Container,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::dsl::parent_sample_container
                    .eq(parent_sample_container.id),
            )
            .load::<Self>(conn)
            .await
    }
}
