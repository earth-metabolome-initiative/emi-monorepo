#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        foreign_key = extraction_container
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::extraction_methods::ExtractionMethod,
        foreign_key = extraction_method
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::extraction_data::extraction_data
)]
pub struct ExtractionDatum {
    pub id: i32,
    pub status: Option<String>,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<::rosetta_uuid::Uuid>,
    pub date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    pub dried_weight: f32,
    pub dried_weight_unit: i32,
    pub extraction_method: Option<i32>,
    pub batch: Option<i32>,
    pub solvent_volume: Option<f32>,
    pub solvent_volume_unit: Option<i32>,
    pub uuid_extraction: Option<::rosetta_uuid::Uuid>,
    pub sample_container: i32,
    pub parent_container: Option<i32>,
    pub parent_sample_container: i32,
    pub extraction_container: Option<i32>,
    pub altemis_tube_id: Option<String>,
    pub altemis_rack_id: Option<String>,
}
impl web_common_traits::prelude::TableName for ExtractionDatum {
    const TABLE_NAME: &'static str = "Extraction_Data";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::extraction_data::ExtractionDatum,
    > for ExtractionDatum
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for ExtractionDatum {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for ExtractionDatum {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl ExtractionDatum {
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
    pub fn dried_weight_unit<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::si_units::SiUnit, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::si_units::SiUnit:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::si_units::SiUnit::read(
            self.dried_weight_unit,
            conn,
        )
    }
    pub fn extraction_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_models::ContainerModel:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(extraction_container) = self.extraction_container else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::container_models::ContainerModel::read(
            extraction_container,
            conn,
        )
        .optional()
    }
    pub fn extraction_method<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::extraction_methods::ExtractionMethod>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::extraction_methods::ExtractionMethod:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(extraction_method) = self.extraction_method else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::extraction_methods::ExtractionMethod::read(
            extraction_method,
            conn,
        )
        .optional()
    }
    pub fn parent_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::containers::Container>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::containers::Container:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_container) = self.parent_container else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::containers::Container::read(parent_container, conn)
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
    pub fn sample_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::containers::Container:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::containers::Container::read(
            self.sample_container,
            conn,
        )
    }
    pub fn solvent_volume_unit<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::si_units::SiUnit>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::si_units::SiUnit:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(solvent_volume_unit) = self.solvent_volume_unit else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::si_units::SiUnit::read(solvent_volume_unit, conn)
            .optional()
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
    #[cfg(feature = "postgres")]
    pub fn from_sample_container(
        sample_container: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::sample_container.eq(sample_container))
            .order_by(extraction_data::id.asc())
            .first::<Self>(conn)
    }
    pub fn from_dried_weight_unit<C>(
        dried_weight_unit: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::dried_weight_unit as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::dried_weight_unit as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::dried_weight_unit as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::dried_weight_unit.eq(dried_weight_unit))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_container(
        parent_container: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::parent_container.eq(parent_container))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_parent_sample_container<C>(
        parent_sample_container: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::parent_sample_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::parent_sample_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::parent_sample_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::extraction_data::extraction_data::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::parent_sample_container.eq(parent_sample_container))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_solvent_volume_unit(
        solvent_volume_unit: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::solvent_volume_unit.eq(solvent_volume_unit))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_created(
        user_created: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::user_created.eq(user_created))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_updated(
        user_updated: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::user_updated.eq(user_updated))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::status.eq(status))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::date_created.eq(date_created))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_updated(
        date_updated: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::date_updated.eq(date_updated))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_uuid_extraction(
        uuid_extraction: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::uuid_extraction.eq(uuid_extraction))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_altemis_tube_id(
        altemis_tube_id: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::altemis_tube_id.eq(altemis_tube_id))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_altemis_rack_id(
        altemis_rack_id: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::extraction_data::extraction_data;
        Self::table()
            .filter(extraction_data::altemis_rack_id.eq(altemis_rack_id))
            .order_by(extraction_data::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<ExtractionDatum> for ExtractionDatum {
    fn as_ref(&self) -> &ExtractionDatum {
        self
    }
}
