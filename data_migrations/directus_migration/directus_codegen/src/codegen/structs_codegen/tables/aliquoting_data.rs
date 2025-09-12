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
        crate::codegen::structs_codegen::tables::si_units::SiUnit,
        foreign_key = aliquot_volume_unit
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data
)]
pub struct AliquotingDatum {
    pub id: i32,
    pub status: String,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<::rosetta_uuid::Uuid>,
    pub date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    pub sample_container: i32,
    pub uuid_aliquot: Option<::rosetta_uuid::Uuid>,
    pub aliquot_volume: f32,
    pub aliquot_volume_unit: i32,
    pub parent_container: i32,
    pub parent_sample_container: i32,
}
impl web_common_traits::prelude::TableName for AliquotingDatum {
    const TABLE_NAME: &'static str = "Aliquoting_Data";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::aliquoting_data::AliquotingDatum,
    > for AliquotingDatum
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for AliquotingDatum {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for AliquotingDatum {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl AliquotingDatum {
    pub fn aliquot_volume_unit<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::si_units::SiUnit, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::si_units::SiUnit:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::si_units::SiUnit::read(
            self.aliquot_volume_unit,
            conn,
        )
    }
    pub fn parent_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::containers::Container:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::containers::Container::read(
            self.parent_container,
            conn,
        )
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

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::sample_container.eq(sample_container))
            .order_by(aliquoting_data::id.asc())
            .first::<Self>(conn)
    }
    pub fn from_parent_container<C>(
        parent_container: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::parent_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::parent_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::parent_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::parent_container.eq(parent_container))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_parent_sample_container<C>(
        parent_sample_container: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::parent_sample_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::parent_sample_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::parent_sample_container as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::parent_sample_container.eq(parent_sample_container))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_created(
        user_created: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::user_created.eq(user_created))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_updated(
        user_updated: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::user_updated.eq(user_updated))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::status.eq(status))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::date_created.eq(date_created))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_updated(
        date_updated: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::date_updated.eq(date_updated))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_uuid_aliquot(
        uuid_aliquot: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_data::aliquoting_data;
        Self::table()
            .filter(aliquoting_data::uuid_aliquot.eq(uuid_aliquot))
            .order_by(aliquoting_data::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<AliquotingDatum> for AliquotingDatum {
    fn as_ref(&self) -> &AliquotingDatum {
        self
    }
}
