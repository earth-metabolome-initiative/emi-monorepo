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
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::directus_extensions::DirectusExtension,
> for DirectusExtension
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{}
impl diesel::Identifiable for DirectusExtension {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusExtension {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusExtension {
    pub fn from_enabled<C>(
        enabled: bool,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions::enabled as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions::enabled as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions::enabled as diesel::expression_methods::EqAll<
                bool,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_extensions::directus_extensions::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
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
        bundle: ::rosetta_uuid::Uuid,
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
