#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        foreign_key = collection
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
        foreign_key = role
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        foreign_key = user_created
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_shares::directus_shares
)]
pub struct DirectusShare {
    pub id: ::rosetta_uuid::Uuid,
    pub name: Option<String>,
    pub collection: String,
    pub item: String,
    pub role: Option<::rosetta_uuid::Uuid>,
    pub password: Option<String>,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub date_start: Option<::rosetta_timestamp::TimestampUTC>,
    pub date_end: Option<::rosetta_timestamp::TimestampUTC>,
    pub times_used: Option<i32>,
    pub max_uses: Option<i32>,
}
impl web_common_traits::prelude::TableName for DirectusShare {
    const TABLE_NAME: &'static str = "directus_shares";
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::directus_shares::DirectusShare,
> for DirectusShare
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{}
impl diesel::Identifiable for DirectusShare {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusShare {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusShare {
    pub fn collection<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection::read(
            self.collection.clone(),
            conn,
        )
    }
    pub fn role<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        use diesel::OptionalExtension;
        let Some(role) = self.role else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::read(
                role,
                conn,
            )
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
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        use diesel::OptionalExtension;
        let Some(user_created) = self.user_created else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
                user_created,
                conn,
            )
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::name.eq(name))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_item(
        item: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::item.eq(item))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_password(
        password: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::password.eq(password))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::date_created.eq(date_created))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_start(
        date_start: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::date_start.eq(date_start))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_end(
        date_end: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::date_end.eq(date_end))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_times_used(
        times_used: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::times_used.eq(times_used))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_max_uses(
        max_uses: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
        Self::table()
            .filter(directus_shares::max_uses.eq(max_uses))
            .order_by(directus_shares::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusShare> for DirectusShare {
    fn as_ref(&self) -> &DirectusShare {
        self
    }
}
