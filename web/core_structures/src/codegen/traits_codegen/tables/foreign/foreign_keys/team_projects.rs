#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TeamProjectForeignKeys {
    pub team: Option<crate::codegen::structs_codegen::tables::teams::Team>,
    pub project: Option<crate::codegen::structs_codegen::tables::projects::Project>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::team_projects::TeamProject
{
    type ForeignKeys = TeamProjectForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Team(self.team_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(self.project_id),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.team.is_some() && foreign_keys.project.is_some()
    }
    fn update(
        &self,
        foreign_keys: &mut Self::ForeignKeys,
        row: Self::Row,
        crud: web_common_traits::crud::CRUD,
    ) -> bool {
        let mut updated = false;
        match (row, crud) {
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.project_id == projects.id {
                    foreign_keys.project = Some(projects);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.project_id == projects.id {
                    foreign_keys.project = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Team(teams),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.team_id == teams.id {
                    foreign_keys.team = Some(teams);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Team(teams),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.team_id == teams.id {
                    foreign_keys.team = None;
                    updated = true;
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for TeamProjectForeignKeys {}
