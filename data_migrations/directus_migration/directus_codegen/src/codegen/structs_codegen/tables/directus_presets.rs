#[derive(Debug, Clone, PartialEq)]
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
    table_name = crate::codegen::diesel_codegen::tables::directus_presets::directus_presets
)]
pub struct DirectusPreset {
    pub id: i32,
    pub bookmark: Option<String>,
    pub user: Option<::rosetta_uuid::Uuid>,
    pub role: Option<::rosetta_uuid::Uuid>,
    pub collection: Option<String>,
    pub search: Option<String>,
    pub layout: Option<String>,
    pub layout_query: Option<::serde_json::Value>,
    pub layout_options: Option<::serde_json::Value>,
    pub refresh_interval: Option<i32>,
    pub filter: Option<::serde_json::Value>,
    pub icon: Option<String>,
    pub color: Option<String>,
}
impl web_common_traits::prelude::TableName for DirectusPreset {
    const TABLE_NAME: &'static str = "directus_presets";
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::directus_presets::DirectusPreset,
> for DirectusPreset
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for DirectusPreset {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusPreset {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusPreset {
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
    pub fn user<C: diesel::connection::LoadConnection>(
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
        let Some(user) = self.user else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
                user,
                conn,
            )
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_bookmark(
        bookmark: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_presets::directus_presets;
        Self::table()
            .filter(directus_presets::bookmark.eq(bookmark))
            .order_by(directus_presets::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_collection(
        collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_presets::directus_presets;
        Self::table()
            .filter(directus_presets::collection.eq(collection))
            .order_by(directus_presets::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_search(
        search: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_presets::directus_presets;
        Self::table()
            .filter(directus_presets::search.eq(search))
            .order_by(directus_presets::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_layout(
        layout: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_presets::directus_presets;
        Self::table()
            .filter(directus_presets::layout.eq(layout))
            .order_by(directus_presets::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_refresh_interval(
        refresh_interval: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_presets::directus_presets;
        Self::table()
            .filter(directus_presets::refresh_interval.eq(refresh_interval))
            .order_by(directus_presets::id.asc())
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
        use crate::codegen::diesel_codegen::tables::directus_presets::directus_presets;
        Self::table()
            .filter(directus_presets::icon.eq(icon))
            .order_by(directus_presets::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_color(
        color: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_presets::directus_presets;
        Self::table()
            .filter(directus_presets::color.eq(color))
            .order_by(directus_presets::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusPreset> for DirectusPreset {
    fn as_ref(&self) -> &DirectusPreset {
        self
    }
}
