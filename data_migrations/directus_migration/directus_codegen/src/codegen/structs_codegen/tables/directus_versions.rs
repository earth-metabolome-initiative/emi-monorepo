#[derive(Debug, Clone, PartialEq)]
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
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        foreign_key = collection
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_versions::directus_versions
)]
pub struct DirectusVersion {
    pub id: ::rosetta_uuid::Uuid,
    pub key: String,
    pub name: Option<String>,
    pub collection: String,
    pub item: String,
    pub hash: Option<String>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub user_updated: Option<::rosetta_uuid::Uuid>,
    pub delta: Option<::serde_json::Value>,
}
impl web_common_traits::prelude::TableName for DirectusVersion {
    const TABLE_NAME: &'static str = "directus_versions";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion,
    > for DirectusVersion
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for DirectusVersion {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusVersion {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusVersion {
    pub fn collection<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection::read(
            self.collection.clone(),
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
    pub fn from_user_created(
        user_created: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::user_created.eq(user_created))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_updated(
        user_updated: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::user_updated.eq(user_updated))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_key(
        key: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::key.eq(key))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::name.eq(name))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_item(
        item: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::item.eq(item))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_hash(
        hash: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::hash.eq(hash))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::date_created.eq(date_created))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_updated(
        date_updated: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
        Self::table()
            .filter(directus_versions::date_updated.eq(date_updated))
            .order_by(directus_versions::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusVersion> for DirectusVersion {
    fn as_ref(&self) -> &DirectusVersion {
        self
    }
}
