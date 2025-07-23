#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliquotingProcedureModelForeignKeys {
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub aliquoted_from: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub procedure_aliquoted_from: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub aliquoted_into: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub procedure_aliquoted_into: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub aliquoted_with: Option<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
    >,
    pub procedure_aliquoted_with: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub pipette_tip: Option<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    >,
    pub procedure_pipette_tip: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub aliquoting_procedure_models_aliquoted_with_pipette_tip_fkey: Option<
        crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel {
    type ForeignKeys = AliquotingProcedureModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                        self.procedure_model_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.aliquoted_from,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.procedure_aliquoted_from,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.aliquoted_into,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.procedure_aliquoted_into,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                        self.aliquoted_with,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.procedure_aliquoted_with,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteTipModel(
                        self.pipette_tip,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.procedure_pipette_tip,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CompatibilityRule((
                        self.aliquoted_with,
                        self.pipette_tip,
                    )),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_model.is_some() && foreign_keys.aliquoted_from.is_some()
            && foreign_keys.procedure_aliquoted_from.is_some()
            && foreign_keys.aliquoted_into.is_some()
            && foreign_keys.procedure_aliquoted_into.is_some()
            && foreign_keys.aliquoted_with.is_some()
            && foreign_keys.procedure_aliquoted_with.is_some()
            && foreign_keys.pipette_tip.is_some()
            && foreign_keys.procedure_pipette_tip.is_some()
            && foreign_keys
                .aliquoting_procedure_models_aliquoted_with_pipette_tip_fkey
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
                if self.aliquoted_with == compatibility_rules.left_trackable_id
                    && self.pipette_tip == compatibility_rules.right_trackable_id
                {
                    foreign_keys
                        .aliquoting_procedure_models_aliquoted_with_pipette_tip_fkey = Some(
                        compatibility_rules,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CompatibilityRule(compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_with == compatibility_rules.left_trackable_id
                    && self.pipette_tip == compatibility_rules.right_trackable_id
                {
                    foreign_keys
                        .aliquoting_procedure_models_aliquoted_with_pipette_tip_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.aliquoted_with == pipette_models.id {
                    foreign_keys.aliquoted_with = Some(pipette_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_with == pipette_models.id {
                    foreign_keys.aliquoted_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteTipModel(pipette_tip_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.pipette_tip == pipette_tip_models.id {
                    foreign_keys.pipette_tip = Some(pipette_tip_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteTipModel(pipette_tip_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.pipette_tip == pipette_tip_models.id {
                    foreign_keys.pipette_tip = None;
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
                if self.procedure_aliquoted_from == procedure_model_trackables.id {
                    foreign_keys.procedure_aliquoted_from = Some(
                        procedure_model_trackables.clone(),
                    );
                    updated = true;
                }
                if self.procedure_aliquoted_into == procedure_model_trackables.id {
                    foreign_keys.procedure_aliquoted_into = Some(
                        procedure_model_trackables.clone(),
                    );
                    updated = true;
                }
                if self.procedure_aliquoted_with == procedure_model_trackables.id {
                    foreign_keys.procedure_aliquoted_with = Some(
                        procedure_model_trackables.clone(),
                    );
                    updated = true;
                }
                if self.procedure_pipette_tip == procedure_model_trackables.id {
                    foreign_keys.procedure_pipette_tip = Some(
                        procedure_model_trackables.clone(),
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_aliquoted_from == procedure_model_trackables.id {
                    foreign_keys.procedure_aliquoted_from = None;
                    updated = true;
                }
                if self.procedure_aliquoted_into == procedure_model_trackables.id {
                    foreign_keys.procedure_aliquoted_into = None;
                    updated = true;
                }
                if self.procedure_aliquoted_with == procedure_model_trackables.id {
                    foreign_keys.procedure_aliquoted_with = None;
                    updated = true;
                }
                if self.procedure_pipette_tip == procedure_model_trackables.id {
                    foreign_keys.procedure_pipette_tip = None;
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
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.aliquoted_from == volumetric_container_models.id {
                    foreign_keys.aliquoted_from = Some(volumetric_container_models);
                    updated = true;
                }
                if self.aliquoted_into == volumetric_container_models.id {
                    foreign_keys.aliquoted_into = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_from == volumetric_container_models.id {
                    foreign_keys.aliquoted_from = None;
                    updated = true;
                }
                if self.aliquoted_into == volumetric_container_models.id {
                    foreign_keys.aliquoted_into = None;
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
impl web_common_traits::prelude::ForeignKeys for AliquotingProcedureModelForeignKeys {}
