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
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_translations::directus_translations
)]
pub struct DirectusTranslation {
    pub id: ::rosetta_uuid::Uuid,
    pub language: String,
    pub key: String,
    pub value: String,
}
impl web_common_traits::prelude::TableName for DirectusTranslation {
    const TABLE_NAME: &'static str = "directus_translations";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_translations::DirectusTranslation,
    > for DirectusTranslation
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for DirectusTranslation {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusTranslation {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusTranslation {
    #[cfg(feature = "postgres")]
    pub fn from_language(
        language: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_translations::directus_translations;
        Self::table()
            .filter(directus_translations::language.eq(language))
            .order_by(directus_translations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_key(
        key: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_translations::directus_translations;
        Self::table()
            .filter(directus_translations::key.eq(key))
            .order_by(directus_translations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_value(
        value: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_translations::directus_translations;
        Self::table()
            .filter(directus_translations::value.eq(value))
            .order_by(directus_translations::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusTranslation> for DirectusTranslation {
    fn as_ref(&self) -> &DirectusTranslation {
        self
    }
}
