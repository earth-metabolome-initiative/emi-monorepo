#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StorageProcedureModelForeignKeys {
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub parent_container: Option<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    >,
    pub procedure_parent_container: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub child_container: Option<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    >,
    pub procedure_child_container: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub storage_procedure_models_parent_container_id_child_contain_fkey: Option<
        crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel
{
    type ForeignKeys = StorageProcedureModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                self.procedure_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                self.parent_container_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                self.procedure_parent_container_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                self.child_container_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                self.procedure_child_container_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CompatibilityRule((
                self.parent_container_id,
                self.child_container_id,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_model.is_some()
            && foreign_keys.parent_container.is_some()
            && foreign_keys.procedure_parent_container.is_some()
            && foreign_keys.child_container.is_some()
            && foreign_keys.procedure_child_container.is_some()
            && foreign_keys
                .storage_procedure_models_parent_container_id_child_contain_fkey
                .is_some()
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
                crate::codegen::tables::row::Row::CompatibilityRule(compatibility_rules),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_container_id == compatibility_rules.left_trackable_id
                    && self.child_container_id == compatibility_rules.right_trackable_id
                {
                    foreign_keys.storage_procedure_models_parent_container_id_child_contain_fkey =
                        Some(compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CompatibilityRule(compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_container_id == compatibility_rules.left_trackable_id
                    && self.child_container_id == compatibility_rules.right_trackable_id
                {
                    foreign_keys.storage_procedure_models_parent_container_id_child_contain_fkey =
                        None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_parent_container_id == procedure_model_trackables.id {
                    foreign_keys.procedure_parent_container =
                        Some(procedure_model_trackables.clone());
                    updated = true;
                }
                if self.procedure_child_container_id == procedure_model_trackables.id {
                    foreign_keys.procedure_child_container =
                        Some(procedure_model_trackables.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_parent_container_id == procedure_model_trackables.id {
                    foreign_keys.procedure_parent_container = None;
                    updated = true;
                }
                if self.procedure_child_container_id == procedure_model_trackables.id {
                    foreign_keys.procedure_child_container = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_model_id == procedure_models.id {
                    foreign_keys.procedure_model = Some(procedure_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_model_id == procedure_models.id {
                    foreign_keys.procedure_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_container_id == trackables.id {
                    foreign_keys.parent_container = Some(trackables.clone());
                    updated = true;
                }
                if self.child_container_id == trackables.id {
                    foreign_keys.child_container = Some(trackables.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_container_id == trackables.id {
                    foreign_keys.parent_container = None;
                    updated = true;
                }
                if self.child_container_id == trackables.id {
                    foreign_keys.child_container = None;
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
impl web_common_traits::prelude::ForeignKeys for StorageProcedureModelForeignKeys {}
