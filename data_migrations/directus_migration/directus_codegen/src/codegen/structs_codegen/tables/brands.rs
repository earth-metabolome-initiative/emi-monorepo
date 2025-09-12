#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::brands::brands)]
pub struct Brand {
    pub id: i32,
    pub status: String,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<::rosetta_uuid::Uuid>,
    pub date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    pub brand: String,
}
impl web_common_traits::prelude::TableName for Brand {
    const TABLE_NAME: &'static str = "Brands";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::brands::Brand,
    > for Brand
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for Brand {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for Brand {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl Brand {
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
    pub fn from_brand(
        brand: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::brands::brands;
        Self::table().filter(brands::brand.eq(brand)).order_by(brands::id.asc()).first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_created(
        user_created: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::brands::brands;
        Self::table()
            .filter(brands::user_created.eq(user_created))
            .order_by(brands::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_updated(
        user_updated: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::brands::brands;
        Self::table()
            .filter(brands::user_updated.eq(user_updated))
            .order_by(brands::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::brands::brands;
        Self::table()
            .filter(brands::status.eq(status))
            .order_by(brands::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::brands::brands;
        Self::table()
            .filter(brands::date_created.eq(date_created))
            .order_by(brands::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_updated(
        date_updated: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::brands::brands;
        Self::table()
            .filter(brands::date_updated.eq(date_updated))
            .order_by(brands::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<Brand> for Brand {
    fn as_ref(&self) -> &Brand {
        self
    }
}
