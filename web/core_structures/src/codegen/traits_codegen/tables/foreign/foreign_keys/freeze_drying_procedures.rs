#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FreezeDryingProcedureForeignKeys {
    pub freeze_dried_container: Option<crate::VolumetricContainer>,
    pub freeze_dried_container_model: Option<crate::VolumetricContainerModel>,
    pub freeze_dried_with: Option<crate::FreezeDryer>,
    pub freeze_dried_with_model: Option<crate::FreezeDryerModel>,
    pub freeze_drying_procedures_freeze_dried_with_model_freeze_dr_fkey:
        Option<crate::AssetCompatibilityRule>,
    pub procedure: Option<crate::Procedure>,
    pub procedure_freeze_dried_container: Option<crate::ProcedureAsset>,
    pub procedure_freeze_dried_with: Option<crate::ProcedureAsset>,
    pub procedure_template: Option<crate::FreezeDryingProcedureTemplate>,
    pub procedure_template_freeze_dried_container_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_freeze_dried_with_model: Option<crate::ProcedureTemplateAssetModel>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::FreezeDryingProcedure {
    type ForeignKeys = FreezeDryingProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.freeze_dried_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.freeze_dried_container_model,
            ),
        ));
        if let Some(freeze_dried_with) = self.freeze_dried_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryer(
                    freeze_dried_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryerModel(
                self.freeze_dried_with_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.freeze_dried_with_model,
                self.freeze_dried_container_model,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_freeze_dried_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_freeze_dried_with,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_freeze_dried_container_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_freeze_dried_with_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.freeze_dried_container.is_some()
            && foreign_keys.freeze_dried_container_model.is_some()
            && (foreign_keys.freeze_dried_with.is_some() || self.freeze_dried_with.is_some())
            && foreign_keys.freeze_dried_with_model.is_some()
            && foreign_keys
                .freeze_drying_procedures_freeze_dried_with_model_freeze_dr_fkey
                .is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_freeze_dried_container.is_some()
            && foreign_keys.procedure_freeze_dried_with.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure_template_freeze_dried_container_model.is_some()
            && foreign_keys.procedure_template_freeze_dried_with_model.is_some()
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
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.freeze_dried_with_model == asset_compatibility_rules.left_asset_model
                    && self.freeze_dried_container_model
                        == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.freeze_drying_procedures_freeze_dried_with_model_freeze_dr_fkey =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dried_with_model == asset_compatibility_rules.left_asset_model
                    && self.freeze_dried_container_model
                        == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.freeze_drying_procedures_freeze_dried_with_model_freeze_dr_fkey =
                        None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryerModel(freeze_dryer_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.freeze_dried_with_model == freeze_dryer_models.id {
                    foreign_keys.freeze_dried_with_model = Some(freeze_dryer_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryerModel(freeze_dryer_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dried_with_model == freeze_dryer_models.id {
                    foreign_keys.freeze_dried_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryer(freeze_dryers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .freeze_dried_with
                    .is_some_and(|freeze_dried_with| freeze_dried_with == freeze_dryers.id)
                {
                    foreign_keys.freeze_dried_with = Some(freeze_dryers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryer(freeze_dryers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .freeze_dried_with
                    .is_some_and(|freeze_dried_with| freeze_dried_with == freeze_dryers.id)
                {
                    foreign_keys.freeze_dried_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryingProcedureTemplate(
                    freeze_drying_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == freeze_drying_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(freeze_drying_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezeDryingProcedureTemplate(
                    freeze_drying_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == freeze_drying_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_freeze_dried_container == procedure_assets.id {
                    foreign_keys.procedure_freeze_dried_container = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_freeze_dried_with == procedure_assets.id {
                    foreign_keys.procedure_freeze_dried_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_freeze_dried_container == procedure_assets.id {
                    foreign_keys.procedure_freeze_dried_container = None;
                    updated = true;
                }
                if self.procedure_freeze_dried_with == procedure_assets.id {
                    foreign_keys.procedure_freeze_dried_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template_freeze_dried_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_freeze_dried_container_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_freeze_dried_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_freeze_dried_with_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template_freeze_dried_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_freeze_dried_container_model = None;
                    updated = true;
                }
                if self.procedure_template_freeze_dried_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_freeze_dried_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = Some(procedures);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = None;
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
                if self.freeze_dried_container_model == volumetric_container_models.id {
                    foreign_keys.freeze_dried_container_model = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dried_container_model == volumetric_container_models.id {
                    foreign_keys.freeze_dried_container_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.freeze_dried_container == volumetric_containers.id {
                    foreign_keys.freeze_dried_container = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.freeze_dried_container == volumetric_containers.id {
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
impl web_common_traits::prelude::ForeignKeys for FreezeDryingProcedureForeignKeys {}
