#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrackableForeignKeys {
    pub trackable_category:
        Option<crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory>,
    pub container_model:
        Option<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    pub project: Option<crate::codegen::structs_codegen::tables::projects::Project>,
    pub trackable_state:
        Option<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::trackables::Trackable
{
    type ForeignKeys = TrackableForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TrackableCategory(
                self.trackable_category_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                self.container_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(self.project_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TrackableState(
                self.trackable_state_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.trackable_category.is_some()
            && foreign_keys.container_model.is_some()
            && foreign_keys.project.is_some()
            && foreign_keys.trackable_state.is_some()
            && foreign_keys.created_by.is_some()
            && foreign_keys.updated_by.is_some()
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
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if container_models.id == self.container_model_id {
                    foreign_keys.container_model = Some(container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if container_models.id == self.container_model_id {
                    foreign_keys.container_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if projects.id == self.project_id {
                    foreign_keys.project = Some(projects);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Project(projects),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if projects.id == self.project_id {
                    foreign_keys.project = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::TrackableCategory(trackable_categories),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if trackable_categories.id == self.trackable_category_id {
                    foreign_keys.trackable_category = Some(trackable_categories);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::TrackableCategory(trackable_categories),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if trackable_categories.id == self.trackable_category_id {
                    foreign_keys.trackable_category = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::TrackableState(trackable_states),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if trackable_states.id == self.trackable_state_id {
                    foreign_keys.trackable_state = Some(trackable_states);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::TrackableState(trackable_states),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if trackable_states.id == self.trackable_state_id {
                    foreign_keys.trackable_state = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = Some(users.clone());
                    updated = true;
                }
                if users.id == self.updated_by {
                    foreign_keys.updated_by = Some(users.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = None;
                    updated = true;
                }
                if users.id == self.updated_by {
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
impl web_common_traits::prelude::ForeignKeys for TrackableForeignKeys {}
