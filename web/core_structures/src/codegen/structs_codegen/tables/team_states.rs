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
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(belongs_to(crate::Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::team_states::team_states)]
pub struct TeamState {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for TeamState {
    const TABLE_NAME: &'static str = "team_states";
}
impl web_common_traits::prelude::ExtensionTable<crate::TeamState> for TeamState where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>
{
}
impl diesel::Identifiable for TeamState {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl TeamState {
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Color, diesel::result::Error>
    where
        crate::Color: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Color::read(self.color_id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::team_states::team_states;
        Self::table()
            .filter(team_states::name.eq(name))
            .order_by(team_states::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::team_states::team_states;
        Self::table()
            .filter(team_states::description.eq(description))
            .order_by(team_states::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::team_states::team_states;
        Self::table()
            .filter(team_states::icon.eq(icon))
            .order_by(team_states::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_color_id(
        color_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::team_states::team_states;
        Self::table()
            .filter(team_states::color_id.eq(color_id))
            .order_by(team_states::id.asc())
            .first::<Self>(conn)
    }
}
impl AsRef<TeamState> for TeamState {
    fn as_ref(&self) -> &TeamState {
        self
    }
}
