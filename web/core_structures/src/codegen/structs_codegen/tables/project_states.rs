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
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::colors::Color,
        foreign_key = color_id
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::project_states::project_states
)]
pub struct ProjectState {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for ProjectState {
    const TABLE_NAME: &'static str = "project_states";
}
impl<'a> From<&'a ProjectState>
    for web_common_traits::database::IdOrBuilder<
        i16,
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectStateBuilder,
    >
{
    fn from(value: &'a ProjectState) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::project_states::ProjectState,
    > for ProjectState
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>,
{
}
impl diesel::Identifiable for ProjectState {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl ProjectState {
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::colors::Color:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::colors::Color::read(self.color_id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::project_states::project_states;
        Self::table()
            .filter(project_states::name.eq(name))
            .order_by(project_states::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::project_states::project_states;
        Self::table()
            .filter(project_states::description.eq(description))
            .order_by(project_states::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::project_states::project_states;
        Self::table()
            .filter(project_states::icon.eq(icon))
            .order_by(project_states::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_color_id(
        color_id: i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::project_states::project_states;
        Self::table()
            .filter(project_states::color_id.eq(color_id))
            .order_by(project_states::id.asc())
            .first::<Self>(conn)
    }
}
impl AsRef<ProjectState> for ProjectState {
    fn as_ref(&self) -> &ProjectState {
        self
    }
}
