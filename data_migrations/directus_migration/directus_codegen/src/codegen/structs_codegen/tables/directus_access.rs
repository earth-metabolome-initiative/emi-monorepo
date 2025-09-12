#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
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
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
        foreign_key = policy
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
        foreign_key = user
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_access::directus_access
)]
pub struct DirectusAccess {
    pub id: ::rosetta_uuid::Uuid,
    pub role: Option<::rosetta_uuid::Uuid>,
    pub user: Option<::rosetta_uuid::Uuid>,
    pub policy: ::rosetta_uuid::Uuid,
    pub sort: Option<i32>,
}
impl web_common_traits::prelude::TableName for DirectusAccess {
    const TABLE_NAME: &'static str = "directus_access";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_access::DirectusAccess,
    > for DirectusAccess
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for DirectusAccess {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusAccess {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusAccess {
    pub fn policy<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy::read(
            self.policy,
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
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(role) = self.role else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::read(role, conn)
            .optional()
    }
    pub fn user<C: diesel::connection::LoadConnection>(
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
        let Some(user) = self.user else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(user, conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_sort(
        sort: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_access::directus_access;
        Self::table()
            .filter(directus_access::sort.eq(sort))
            .order_by(directus_access::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusAccess> for DirectusAccess {
    fn as_ref(&self) -> &DirectusAccess {
        self
    }
}
