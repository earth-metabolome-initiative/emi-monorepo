#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedProcedureModelTrackableForeignKeys {
    pub parent_procedure_model: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub child: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub child_procedure_model: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub child_trackable: Option<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    >,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub parent: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub parent_trackable: Option<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable {
    type ForeignKeys = SharedProcedureModelTrackableForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                        self.parent_procedure_model_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.child_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                        self.child_procedure_model_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                        self.child_trackable_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                        self.created_by,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.parent_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                        self.parent_trackable_id,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.parent_procedure_model.is_some() && foreign_keys.child.is_some()
            && foreign_keys.child_procedure_model.is_some()
            && foreign_keys.child_trackable.is_some()
            && foreign_keys.created_by.is_some() && foreign_keys.parent.is_some()
            && foreign_keys.parent_trackable.is_some()
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
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.child_id == procedure_model_trackables.id {
                    foreign_keys.child = Some(procedure_model_trackables.clone());
                    updated = true;
                }
                if self.parent_id == procedure_model_trackables.id {
                    foreign_keys.parent = Some(procedure_model_trackables.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.child_id == procedure_model_trackables.id {
                    foreign_keys.child = None;
                    updated = true;
                }
                if self.parent_id == procedure_model_trackables.id {
                    foreign_keys.parent = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_procedure_model_id == procedure_models.id {
                    foreign_keys.parent_procedure_model = Some(procedure_models.clone());
                    updated = true;
                }
                if self.child_procedure_model_id == procedure_models.id {
                    foreign_keys.child_procedure_model = Some(procedure_models.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_procedure_model_id == procedure_models.id {
                    foreign_keys.parent_procedure_model = None;
                    updated = true;
                }
                if self.child_procedure_model_id == procedure_models.id {
                    foreign_keys.child_procedure_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.child_trackable_id == trackables.id {
                    foreign_keys.child_trackable = Some(trackables.clone());
                    updated = true;
                }
                if self.parent_trackable_id == trackables.id {
                    foreign_keys.parent_trackable = Some(trackables.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.child_trackable_id == trackables.id {
                    foreign_keys.child_trackable = None;
                    updated = true;
                }
                if self.parent_trackable_id == trackables.id {
                    foreign_keys.parent_trackable = None;
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
                    foreign_keys.created_by = Some(users);
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
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for SharedProcedureModelTrackableForeignKeys {}
