#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(version))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_migrations::directus_migrations
)]
pub struct DirectusMigration {
    pub version: String,
    pub name: String,
    pub timestamp: Option<::rosetta_timestamp::TimestampUTC>,
}
impl web_common_traits::prelude::TableName for DirectusMigration {
    const TABLE_NAME: &'static str = "directus_migrations";
}
impl diesel::Identifiable for DirectusMigration {
    type Id = String;
    fn id(self) -> Self::Id {
        self.version
    }
}
impl DirectusMigration {
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_migrations::directus_migrations;
        Self::table()
            .filter(directus_migrations::name.eq(name))
            .order_by(directus_migrations::version.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_timestamp(
        timestamp: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_migrations::directus_migrations;
        Self::table()
            .filter(directus_migrations::timestamp.eq(timestamp))
            .order_by(directus_migrations::version.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusMigration> for DirectusMigration {
    fn as_ref(&self) -> &DirectusMigration {
        self
    }
}
