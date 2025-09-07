#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SupernatantProcedureTemplateForeignKeys {
    pub supernatant_pm_compatibility_rules: Option<crate::AssetCompatibilityRule>,
    pub procedure_template_pipette_tip_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_stratified_source_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_supernatant_destination_model:
        Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_template_transferred_with_model: Option<crate::ProcedureTemplateAssetModel>,
    pub supernatant_destination_model: Option<crate::VolumetricContainerModel>,
    pub pipette_tip_model: Option<crate::PipetteTipModel>,
    pub procedure_template: Option<crate::ProcedureTemplate>,
    pub stratified_source_model: Option<crate::VolumetricContainerModel>,
    pub transferred_with_model: Option<crate::PipetteModel>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::SupernatantProcedureTemplate {
    type ForeignKeys = SupernatantProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.transferred_with_model,
                self.pipette_tip_model,
            )),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_pipette_tip_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_stratified_source_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_supernatant_destination_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_transferred_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.supernatant_destination_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteTipModel(
                self.pipette_tip_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.stratified_source_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                self.transferred_with_model,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.supernatant_pm_compatibility_rules.is_some()
            && foreign_keys.procedure_template_pipette_tip_model.is_some()
            && foreign_keys.procedure_template_stratified_source_model.is_some()
            && foreign_keys.procedure_template_supernatant_destination_model.is_some()
            && foreign_keys.procedure_template_transferred_with_model.is_some()
            && foreign_keys.supernatant_destination_model.is_some()
            && foreign_keys.pipette_tip_model.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.stratified_source_model.is_some()
            && foreign_keys.transferred_with_model.is_some()
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
                if self.transferred_with_model == asset_compatibility_rules.left_asset_model
                    && self.pipette_tip_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.supernatant_pm_compatibility_rules =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.transferred_with_model == asset_compatibility_rules.left_asset_model
                    && self.pipette_tip_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.supernatant_pm_compatibility_rules = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.transferred_with_model == pipette_models.id {
                    foreign_keys.transferred_with_model = Some(pipette_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.transferred_with_model == pipette_models.id {
                    foreign_keys.transferred_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteTipModel(pipette_tip_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.pipette_tip_model == pipette_tip_models.id {
                    foreign_keys.pipette_tip_model = Some(pipette_tip_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteTipModel(pipette_tip_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.pipette_tip_model == pipette_tip_models.id {
                    foreign_keys.pipette_tip_model = None;
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
                if self.procedure_template_pipette_tip_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_pipette_tip_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_stratified_source_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stratified_source_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_supernatant_destination_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_supernatant_destination_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_transferred_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_transferred_with_model =
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
                if self.procedure_template_pipette_tip_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_pipette_tip_model = None;
                    updated = true;
                }
                if self.procedure_template_stratified_source_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stratified_source_model = None;
                    updated = true;
                }
                if self.procedure_template_supernatant_destination_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_supernatant_destination_model = None;
                    updated = true;
                }
                if self.procedure_template_transferred_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_transferred_with_model = None;
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
                if self.supernatant_destination_model == volumetric_container_models.id {
                    foreign_keys.supernatant_destination_model = Some(volumetric_container_models);
                    updated = true;
                }
                if self.stratified_source_model == volumetric_container_models.id {
                    foreign_keys.stratified_source_model = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.supernatant_destination_model == volumetric_container_models.id {
                    foreign_keys.supernatant_destination_model = None;
                    updated = true;
                }
                if self.stratified_source_model == volumetric_container_models.id {
                    foreign_keys.stratified_source_model = None;
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
impl web_common_traits::prelude::ForeignKeys for SupernatantProcedureTemplateForeignKeys {}
