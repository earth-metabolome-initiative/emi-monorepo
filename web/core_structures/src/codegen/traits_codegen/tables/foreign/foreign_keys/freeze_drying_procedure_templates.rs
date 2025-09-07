#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FreezeDryingProcedureTemplateForeignKeys {
    pub procedure_template_freeze_dried_container_model: Option<crate::ProcedureTemplateAssetModel>,
    pub freeze_dried_container_model: Option<crate::VolumetricContainerModel>,
    pub freeze_drying_procedure_templ_freeze_dried_with_model_free_fkey:
        Option<crate::AssetCompatibilityRule>,
    pub procedure_template_freeze_dried_with_model: Option<crate::ProcedureTemplateAssetModel>,
    pub freeze_dried_with_model: Option<crate::FreezeDryerModel>,
    pub procedure_template: Option<crate::ProcedureTemplate>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::FreezeDryingProcedureTemplate {
    type ForeignKeys = FreezeDryingProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_freeze_dried_container_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.freeze_dried_container_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.freeze_dried_with_model,
                self.freeze_dried_container_model,
            )),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_freeze_dried_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryerModel(
                self.freeze_dried_with_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.procedure_template,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template_freeze_dried_container_model.is_some()
            && foreign_keys.freeze_dried_container_model.is_some()
            && foreign_keys
                .freeze_drying_procedure_templ_freeze_dried_with_model_free_fkey
                .is_some()
            && foreign_keys.procedure_template_freeze_dried_with_model.is_some()
            && foreign_keys.freeze_dried_with_model.is_some()
            && foreign_keys.procedure_template.is_some()
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
                    foreign_keys.freeze_drying_procedure_templ_freeze_dried_with_model_free_fkey =
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
                    foreign_keys.freeze_drying_procedure_templ_freeze_dried_with_model_free_fkey =
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
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
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
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for FreezeDryingProcedureTemplateForeignKeys {}
