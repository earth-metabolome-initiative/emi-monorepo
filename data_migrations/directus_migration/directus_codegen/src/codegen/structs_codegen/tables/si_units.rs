#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::si_units::si_units)]
pub struct SiUnit {
    pub id: i32,
    pub status: Option<String>,
    pub user_created: Option<::rosetta_uuid::Uuid>,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_updated: Option<::rosetta_uuid::Uuid>,
    pub date_updated: Option<::rosetta_timestamp::TimestampUTC>,
    pub unit_name: String,
    pub symbol: String,
    pub base_unit: String,
    pub multiplication_factor: f32,
}
impl web_common_traits::prelude::TableName for SiUnit {
    const TABLE_NAME: &'static str = "SI_Units";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::si_units::SiUnit,
    > for SiUnit
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for SiUnit {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for SiUnit {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl SiUnit {
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
    pub fn user_updated<C: diesel::connection::LoadConnection>(
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
        let Some(user_updated) = self.user_updated else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
            user_updated,
            conn,
        )
        .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_unit_name(
        unit_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::unit_name.eq(unit_name))
            .order_by(si_units::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_created(
        user_created: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::user_created.eq(user_created))
            .order_by(si_units::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_updated(
        user_updated: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::user_updated.eq(user_updated))
            .order_by(si_units::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::status.eq(status))
            .order_by(si_units::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::date_created.eq(date_created))
            .order_by(si_units::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_updated(
        date_updated: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::date_updated.eq(date_updated))
            .order_by(si_units::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_symbol(
        symbol: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::symbol.eq(symbol))
            .order_by(si_units::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_base_unit(
        base_unit: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::si_units::si_units;
        Self::table()
            .filter(si_units::base_unit.eq(base_unit))
            .order_by(si_units::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<SiUnit> for SiUnit {
    fn as_ref(&self) -> &SiUnit {
        self
    }
}
