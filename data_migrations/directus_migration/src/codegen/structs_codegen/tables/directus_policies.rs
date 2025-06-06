#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_policies::directus_policies
)]
pub struct DirectusPolicy {
    pub id: ::rosetta_uuid::Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub ip_access: Option<String>,
    pub enforce_tfa: bool,
    pub admin_access: bool,
    pub app_access: bool,
}
impl web_common_traits::prelude::TableName for DirectusPolicy {
    const TABLE_NAME: &'static str = "directus_policies";
}
impl diesel::Identifiable for DirectusPolicy {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusPolicy {
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
        Self::table()
            .filter(directus_policies::name.eq(name))
            .order_by(directus_policies::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
        Self::table()
            .filter(directus_policies::icon.eq(icon))
            .order_by(directus_policies::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
        Self::table()
            .filter(directus_policies::description.eq(description))
            .order_by(directus_policies::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_ip_access(
        ip_access: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
        Self::table()
            .filter(directus_policies::ip_access.eq(ip_access))
            .order_by(directus_policies::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_enforce_tfa(
        enforce_tfa: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
        Self::table()
            .filter(directus_policies::enforce_tfa.eq(enforce_tfa))
            .order_by(directus_policies::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_admin_access(
        admin_access: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
        Self::table()
            .filter(directus_policies::admin_access.eq(admin_access))
            .order_by(directus_policies::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_app_access(
        app_access: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
        Self::table()
            .filter(directus_policies::app_access.eq(app_access))
            .order_by(directus_policies::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusPolicy> for DirectusPolicy {
    fn as_ref(&self) -> &DirectusPolicy {
        self
    }
}
