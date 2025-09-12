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
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
        foreign_key = parent
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_roles::directus_roles
)]
pub struct DirectusRole {
    pub id: ::rosetta_uuid::Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub parent: Option<::rosetta_uuid::Uuid>,
}
impl web_common_traits::prelude::TableName for DirectusRole {
    const TABLE_NAME: &'static str = "directus_roles";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
    > for DirectusRole
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl<C> web_common_traits::prelude::Ancestor<C> for DirectusRole
where
    Self: web_common_traits::prelude::TableName + Sized,
    C: diesel::connection::LoadConnection,
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization
        + diesel::sql_types::HasSqlType<::rosetta_uuid::diesel_impls::Uuid>
        + 'static,
    web_common_traits::prelude::AncestorExists: diesel::deserialize::FromSqlRow<
            diesel::sql_types::Untyped,
            <C as diesel::Connection>::Backend,
        >,
    for<'a> &'a Self: diesel::Identifiable,
    for<'a> <&'a Self as diesel::Identifiable>::Id:
        diesel::serialize::ToSql<::rosetta_uuid::diesel_impls::Uuid, C::Backend>,
{
    const PARENT_ID: &'static str = "parent";
    const ID: &'static str = "id";
    type SqlType = ::rosetta_uuid::diesel_impls::Uuid;
}
impl web_common_traits::prelude::Descendant<DirectusRole> for DirectusRole {
    fn parent(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
        self.parent.as_ref()
    }
}
impl diesel::Identifiable for DirectusRole {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusRole {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusRole {
    pub fn parent<C: diesel::connection::LoadConnection>(
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
        let Some(parent) = self.parent else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::read(parent, conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_roles::directus_roles;
        Self::table()
            .filter(directus_roles::name.eq(name))
            .order_by(directus_roles::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_roles::directus_roles;
        Self::table()
            .filter(directus_roles::icon.eq(icon))
            .order_by(directus_roles::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_roles::directus_roles;
        Self::table()
            .filter(directus_roles::description.eq(description))
            .order_by(directus_roles::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusRole> for DirectusRole {
    fn as_ref(&self) -> &DirectusRole {
        self
    }
}
