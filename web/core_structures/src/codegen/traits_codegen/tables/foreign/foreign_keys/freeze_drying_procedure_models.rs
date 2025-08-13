#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FreezeDryingProcedureModelForeignKeys {
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub freeze_dried_with: Option<
        crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel,
    >,
    pub freeze_dried_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub freeze_drying_pm_compatibility_rules: Option<
        crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel {
    type ForeignKeys = FreezeDryingProcedureModelForeignKeys;
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDrierModel(
                        self.freeze_dried_with,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.freeze_dried_container_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CompatibilityRule((
                        self.freeze_dried_with,
                        self.freeze_dried_container_id,
                    )),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_model.is_some()
            && foreign_keys.freeze_dried_with.is_some()
            && foreign_keys.freeze_dried_container.is_some()
            && foreign_keys.freeze_drying_pm_compatibility_rules.is_some()
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
                if self.freeze_dried_with == compatibility_rules.left_trackable_id
                    && self.freeze_dried_container_id
                        == compatibility_rules.right_trackable_id
                {
                    foreign_keys.freeze_drying_pm_compatibility_rules = Some(
                        compatibility_rules,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CompatibilityRule(compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dried_with == compatibility_rules.left_trackable_id
                    && self.freeze_dried_container_id
                        == compatibility_rules.right_trackable_id
                {
                    foreign_keys.freeze_drying_pm_compatibility_rules = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDrierModel(freeze_drier_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.freeze_dried_with == freeze_drier_models.id {
                    foreign_keys.freeze_dried_with = Some(freeze_drier_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDrierModel(freeze_drier_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dried_with == freeze_drier_models.id {
                    foreign_keys.freeze_dried_with = None;
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
                if self.freeze_dried_container_id == volumetric_container_models.id {
                    foreign_keys.freeze_dried_container = Some(
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
                if self.freeze_dried_container_id == volumetric_container_models.id {
                    foreign_keys.freeze_dried_container = None;
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
impl web_common_traits::prelude::ForeignKeys for FreezeDryingProcedureModelForeignKeys {}
