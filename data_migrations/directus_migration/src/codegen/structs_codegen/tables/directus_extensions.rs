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
    table_name = crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions
)]
pub struct DirectusExtension {
    pub enabled: bool,
    pub id: ::rosetta_uuid::Uuid,
    pub folder: String,
    pub source: String,
    pub bundle: Option<::rosetta_uuid::Uuid>,
}
impl web_common_traits::prelude::TableName for DirectusExtension {
    const TABLE_NAME: &'static str = "directus_extensions";
}
impl diesel::Identifiable for DirectusExtension {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusExtension {
    #[cfg(feature = "postgres")]
    pub fn from_enabled(
        enabled: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions;
        Self::table()
            .filter(directus_extensions::enabled.eq(enabled))
            .order_by(directus_extensions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_folder(
        folder: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions;
        Self::table()
            .filter(directus_extensions::folder.eq(folder))
            .order_by(directus_extensions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_source(
        source: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions;
        Self::table()
            .filter(directus_extensions::source.eq(source))
            .order_by(directus_extensions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_bundle(
        bundle: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions;
        Self::table()
            .filter(directus_extensions::bundle.eq(bundle))
            .order_by(directus_extensions::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusExtension> for DirectusExtension {
    fn as_ref(&self) -> &DirectusExtension {
        self
    }
}
