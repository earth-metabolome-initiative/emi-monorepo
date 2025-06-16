#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectForeignKeys {
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub state: Option<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    pub color: Option<crate::codegen::structs_codegen::tables::colors::Color>,
    pub parent_project: Option<crate::codegen::structs_codegen::tables::projects::Project>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::projects::Project
{
    type ForeignKeys = ProjectForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProjectState(
                self.state_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(self.color_id),
        ));
        if let Some(parent_project_id) = self.parent_project_id {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(
                    parent_project_id,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.created_by.is_some()
            && foreign_keys.updated_by.is_some()
            && foreign_keys.state.is_some()
            && foreign_keys.color.is_some()
            && (foreign_keys.parent_project.is_some() || self.parent_project_id.is_some())
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
                crate::codegen::tables::row::Row::Color(colors),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.color_id == colors.id {
                    foreign_keys.color = Some(colors);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Color(colors),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.color_id == colors.id {
                    foreign_keys.color = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProjectState(project_states),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.state_id == project_states.id {
                    foreign_keys.state = Some(project_states);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProjectState(project_states),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.state_id == project_states.id {
                    foreign_keys.state = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .parent_project_id
                    .is_some_and(|parent_project_id| parent_project_id == projects.id)
                {
                    foreign_keys.parent_project = Some(projects);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .parent_project_id
                    .is_some_and(|parent_project_id| parent_project_id == projects.id)
                {
                    foreign_keys.parent_project = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = Some(users.clone());
                    updated = true;
                }
                if self.updated_by == users.id {
                    foreign_keys.updated_by = Some(users.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = None;
                    updated = true;
                }
                if self.updated_by == users.id {
                    foreign_keys.updated_by = None;
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
impl web_common_traits::prelude::ForeignKeys for ProjectForeignKeys {}
