#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::batches::Batch,
        foreign_key = batch
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::containers::Container,
        foreign_key = parent_sample_container
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::si_units::SiUnit,
        foreign_key = injection_volume_unit
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod,
        foreign_key = injection_method
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::instruments::Instrument,
        foreign_key = instrument_used
    )
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::ms_data::ms_data)]
pub struct MsDatum {
    pub id: i32,
    pub status: Option<String>,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<::rosetta_uuid::Uuid>,
    pub date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    pub uuid_ms_file: Option<::rosetta_uuid::Uuid>,
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
impl web_common_traits::prelude::TableName for MsDatum {
    const TABLE_NAME: &'static str = "MS_Data";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::ms_data::MsDatum,
    > for MsDatum
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for MsDatum {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl MsDatum {
    pub fn batch<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::batches::Batch>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::batches::Batch:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(batch) = self.batch else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::batches::Batch::read(batch, conn).optional()
    }
    pub fn user_created<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(user_created) = self.user_created else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
            user_created,
            conn,
        )
        .optional()
    }
    pub fn user_updated<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(user_updated) = self.user_updated else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
            user_updated,
            conn,
        )
        .optional()
    }
    pub fn parent_sample_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::containers::Container:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::containers::Container::read(
            self.parent_sample_container,
            conn,
        )
    }
    pub fn injection_volume_unit<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::si_units::SiUnit, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::si_units::SiUnit:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::si_units::SiUnit::read(
            self.injection_volume_unit,
            conn,
        )
    }
    pub fn injection_method<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::injection_methods::InjectionMethod::read(
            self.injection_method,
            conn,
        )
    }
    pub fn instrument_used<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::instruments::Instrument:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::instruments::Instrument::read(
            self.instrument_used,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_filename(
        filename: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::filename.eq(filename))
            .order_by(ms_data::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_created(
        user_created: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::user_created.eq(user_created))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_updated(
        user_updated: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::user_updated.eq(user_updated))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::status.eq(status))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::date_created.eq(date_created))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_updated(
        date_updated: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::date_updated.eq(date_updated))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_uuid_ms_file(
        uuid_ms_file: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::uuid_ms_file.eq(uuid_ms_file))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status_comment(
        status_comment: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::status_comment.eq(status_comment))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_injection_volume<C>(
        injection_volume: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ms_data::ms_data::injection_volume as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ms_data::ms_data::injection_volume as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ms_data::ms_data::injection_volume as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::ms_data::ms_data::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::injection_volume.eq(injection_volume))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_converted(
        converted: bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::converted.eq(converted))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_processed(
        processed: bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ms_data::ms_data;
        Self::table()
            .filter(ms_data::processed.eq(processed))
            .order_by(ms_data::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<MsDatum> for MsDatum {
    fn as_ref(&self) -> &MsDatum {
        self
    }
}
