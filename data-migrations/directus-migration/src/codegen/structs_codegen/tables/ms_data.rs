#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "32-column-tables",
    derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)
)]
#[cfg_attr(feature = "32-column-tables", diesel(primary_key(id)))]
#[cfg_attr(
    feature = "32-column-tables",
    diesel(table_name = crate::codegen::diesel_codegen::tables::ms_data::ms_data)
)]
pub struct MsDatum {
    pub id: i32,
    pub status: Option<String>,
    pub user_created: Option<uuid::Uuid>,
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    pub user_updated: Option<uuid::Uuid>,
    pub date_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub uuid_ms_file: Option<uuid::Uuid>,
    pub status_comment: Option<String>,
    pub filename: String,
    pub injection_volume: i32,
    pub injection_volume_unit: i32,
    pub injection_method: i32,
    pub instrument_used: i32,
    pub batch: Option<i32>,
    pub parent_sample_container: i32,
    pub converted: Option<bool>,
    pub processed: Option<bool>,
}
impl MsDatum {
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
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::dsl::user_created
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
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::dsl::user_updated
                    .eq(&user_updated.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn injection_volume_unit(
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
                    .eq(&self.injection_volume_unit),
            )
            .first::<crate::codegen::structs_codegen::tables::si_units::SiUnit>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_injection_volume_unit(
        conn: &mut diesel_async::AsyncPgConnection,
        injection_volume_unit: &crate::codegen::structs_codegen::tables::si_units::SiUnit,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::dsl::injection_volume_unit
                    .eq(injection_volume_unit.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn injection_method(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod::table()
            .filter(
                crate::codegen::diesel_codegen::tables::injection_methods::injection_methods::dsl::id
                    .eq(&self.injection_method),
            )
            .first::<
                crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_injection_method(
        conn: &mut diesel_async::AsyncPgConnection,
        injection_method: &crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::dsl::injection_method
                    .eq(injection_method.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn instrument_used(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::instruments::Instrument::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instruments::instruments::dsl::id
                    .eq(&self.instrument_used),
            )
            .first::<
                crate::codegen::structs_codegen::tables::instruments::Instrument,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_instrument_used(
        conn: &mut diesel_async::AsyncPgConnection,
        instrument_used: &crate::codegen::structs_codegen::tables::instruments::Instrument,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::dsl::instrument_used
                    .eq(instrument_used.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn batch(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::batches::Batch>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(batch) = self.batch.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::batches::Batch::table()
            .filter(
                crate::codegen::diesel_codegen::tables::batches::batches::dsl::id
                    .eq(batch),
            )
            .first::<crate::codegen::structs_codegen::tables::batches::Batch>(conn)
            .await
            .map(Some)
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
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::dsl::batch
                    .eq(batch.id),
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
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::dsl::parent_sample_container
                    .eq(parent_sample_container.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_filename(
        filename: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::ms_data::ms_data::filename,
                    filename,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
