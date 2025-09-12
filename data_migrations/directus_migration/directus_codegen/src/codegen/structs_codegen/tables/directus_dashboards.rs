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
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        foreign_key = user_created
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards
)]
pub struct DirectusDashboard {
    pub id: ::rosetta_uuid::Uuid,
    pub name: String,
    pub icon: String,
    pub note: Option<String>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub color: Option<String>,
}
impl web_common_traits::prelude::TableName for DirectusDashboard {
    const TABLE_NAME: &'static str = "directus_dashboards";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_dashboards::DirectusDashboard,
    > for DirectusDashboard
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for DirectusDashboard {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusDashboard {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusDashboard {
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
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards;
        Self::table()
            .filter(directus_dashboards::name.eq(name))
            .order_by(directus_dashboards::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards;
        Self::table()
            .filter(directus_dashboards::icon.eq(icon))
            .order_by(directus_dashboards::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_note(
        note: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards;
        Self::table()
            .filter(directus_dashboards::note.eq(note))
            .order_by(directus_dashboards::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards;
        Self::table()
            .filter(directus_dashboards::date_created.eq(date_created))
            .order_by(directus_dashboards::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_color(
        color: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards;
        Self::table()
            .filter(directus_dashboards::color.eq(color))
            .order_by(directus_dashboards::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusDashboard> for DirectusDashboard {
    fn as_ref(&self) -> &DirectusDashboard {
        self
    }
}
