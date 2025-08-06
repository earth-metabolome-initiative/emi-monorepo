#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SupernatantProcedureModelForeignKeys {
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub stratified_source: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub supernatant_destination: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub transferred_with: Option<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
    >,
    pub pipette_tip: Option<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    >,
    pub supernatant_procedure_models_transferred_with_pipette_tip_fkey: Option<
        crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel {
    type ForeignKeys = SupernatantProcedureModelForeignKeys;
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
                        self.stratified_source,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.supernatant_destination,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                        self.transferred_with,
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CompatibilityRule((
                        self.transferred_with,
                        self.pipette_tip,
                    )),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_model.is_some()
            && foreign_keys.stratified_source.is_some()
            && foreign_keys.supernatant_destination.is_some()
            && foreign_keys.transferred_with.is_some()
            && foreign_keys.pipette_tip.is_some()
            && foreign_keys
                .supernatant_procedure_models_transferred_with_pipette_tip_fkey
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
                if self.transferred_with == compatibility_rules.left_trackable_id
                    && self.pipette_tip == compatibility_rules.right_trackable_id
                {
                    foreign_keys
                        .supernatant_procedure_models_transferred_with_pipette_tip_fkey = Some(
                        compatibility_rules,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CompatibilityRule(compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.transferred_with == compatibility_rules.left_trackable_id
                    && self.pipette_tip == compatibility_rules.right_trackable_id
                {
                    foreign_keys
                        .supernatant_procedure_models_transferred_with_pipette_tip_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.transferred_with == pipette_models.id {
                    foreign_keys.transferred_with = Some(pipette_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.transferred_with == pipette_models.id {
                    foreign_keys.transferred_with = None;
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
                if self.stratified_source == volumetric_container_models.id {
                    foreign_keys.stratified_source = Some(volumetric_container_models);
                    updated = true;
                }
                if self.supernatant_destination == volumetric_container_models.id {
                    foreign_keys.supernatant_destination = Some(
                        volumetric_container_models,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stratified_source == volumetric_container_models.id {
                    foreign_keys.stratified_source = None;
                    updated = true;
                }
                if self.supernatant_destination == volumetric_container_models.id {
                    foreign_keys.supernatant_destination = None;
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
impl web_common_traits::prelude::ForeignKeys for SupernatantProcedureModelForeignKeys {}
