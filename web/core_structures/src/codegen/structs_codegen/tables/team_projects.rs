#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::projects::Project,
        foreign_key = project_id
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::teams::Team,
        foreign_key = team_id
    )
)]
#[diesel(primary_key(team_id, project_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::team_projects::team_projects
)]
pub struct TeamProject {
    pub team_id: i32,
    pub project_id: i32,
}
impl web_common_traits::prelude::TableName for TeamProject {
    const TABLE_NAME: &'static str = "team_projects";
}
impl<'a> From<&'a TeamProject>
    for web_common_traits::database::IdOrBuilder<
        (i32, i32),
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder,
    >
{
    fn from(value: &'a TeamProject) -> Self {
        web_common_traits::database::IdOrBuilder::Id((value.team_id, value.project_id))
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::team_projects::TeamProject,
    > for TeamProject
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>,
{
}
impl diesel::Identifiable for TeamProject {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.team_id, self.project_id)
    }
}
impl TeamProject {
    pub fn project<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::projects::Project, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::projects::Project:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::projects::Project::read(self.project_id, conn)
    }
    pub fn team<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::teams::Team, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::teams::Team::read(self.team_id, conn)
    }
}
impl AsRef<TeamProject> for TeamProject {
    fn as_ref(&self) -> &TeamProject {
        self
    }
}
