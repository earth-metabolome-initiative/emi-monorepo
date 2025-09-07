#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CentrifugeProcedureTemplateForeignKeys {
    pub procedure_template: Option<crate::ProcedureTemplate>,
    pub centrifuged_with_model: Option<crate::CentrifugeModel>,
    pub procedure_template_centrifuged_with_model: Option<crate::ProcedureTemplateAssetModel>,
    pub centrifuged_container_model: Option<crate::VolumetricContainerModel>,
    pub procedure_template_centrifuged_container_model: Option<crate::ProcedureTemplateAssetModel>,
    pub centrifuge_pm_compatibility_rules: Option<crate::AssetCompatibilityRule>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::CentrifugeProcedureTemplate {
    type ForeignKeys = CentrifugeProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeModel(
                self.centrifuged_with_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_centrifuged_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.centrifuged_container_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_centrifuged_container_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.centrifuged_with_model,
                self.centrifuged_container_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.centrifuged_with_model.is_some()
            && foreign_keys.procedure_template_centrifuged_with_model.is_some()
            && foreign_keys.centrifuged_container_model.is_some()
            && foreign_keys.procedure_template_centrifuged_container_model.is_some()
            && foreign_keys.centrifuge_pm_compatibility_rules.is_some()
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
                if self.centrifuged_with_model == asset_compatibility_rules.left_asset_model
                    && self.centrifuged_container_model
                        == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.centrifuge_pm_compatibility_rules =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.centrifuged_with_model == asset_compatibility_rules.left_asset_model
                    && self.centrifuged_container_model
                        == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.centrifuge_pm_compatibility_rules = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CentrifugeModel(centrifuge_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.centrifuged_with_model == centrifuge_models.id {
                    foreign_keys.centrifuged_with_model = Some(centrifuge_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CentrifugeModel(centrifuge_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.centrifuged_with_model == centrifuge_models.id {
                    foreign_keys.centrifuged_with_model = None;
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
                if self.procedure_template_centrifuged_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_centrifuged_with_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_centrifuged_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_centrifuged_container_model =
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
                if self.procedure_template_centrifuged_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_centrifuged_with_model = None;
                    updated = true;
                }
                if self.procedure_template_centrifuged_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_centrifuged_container_model = None;
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
                if self.centrifuged_container_model == volumetric_container_models.id {
                    foreign_keys.centrifuged_container_model = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.centrifuged_container_model == volumetric_container_models.id {
                    foreign_keys.centrifuged_container_model = None;
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
impl web_common_traits::prelude::ForeignKeys for CentrifugeProcedureTemplateForeignKeys {}
