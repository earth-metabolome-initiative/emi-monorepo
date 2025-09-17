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
        crate::codegen::structs_codegen::tables::field_data::FieldDatum,
        foreign_key = field_data
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data
)]
pub struct DriedSamplesDatum {
    pub id: i32,
    pub status: String,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<::rosetta_uuid::Uuid>,
    pub date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    pub uuid_dried_sample: Option<::rosetta_uuid::Uuid>,
    pub sample_container: i32,
    pub parent_container: Option<i32>,
    pub batch: Option<i32>,
    pub field_data: Option<i32>,
}
impl web_common_traits::prelude::TableName for DriedSamplesDatum {
    const TABLE_NAME: &'static str = "Dried_Samples_Data";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::dried_samples_data::DriedSamplesDatum,
    > for DriedSamplesDatum
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for DriedSamplesDatum {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DriedSamplesDatum {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DriedSamplesDatum {
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
    pub fn field_data<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::field_data::FieldDatum>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::field_data::FieldDatum:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(field_data) = self.field_data else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::field_data::FieldDatum::read(field_data, conn)
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

        use crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data;
        Self::table()
            .filter(dried_samples_data::sample_container.eq(sample_container))
            .order_by(dried_samples_data::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_container(
        parent_container: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data;
        Self::table()
            .filter(dried_samples_data::parent_container.eq(parent_container))
            .order_by(dried_samples_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_created(
        user_created: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data;
        Self::table()
            .filter(dried_samples_data::user_created.eq(user_created))
            .order_by(dried_samples_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_updated(
        user_updated: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data;
        Self::table()
            .filter(dried_samples_data::user_updated.eq(user_updated))
            .order_by(dried_samples_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data;
        Self::table()
            .filter(dried_samples_data::status.eq(status))
            .order_by(dried_samples_data::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_uuid_dried_sample(
        uuid_dried_sample: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::dried_samples_data::dried_samples_data;
        Self::table()
            .filter(dried_samples_data::uuid_dried_sample.eq(uuid_dried_sample))
            .order_by(dried_samples_data::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DriedSamplesDatum> for DriedSamplesDatum {
    fn as_ref(&self) -> &DriedSamplesDatum {
        self
    }
}
