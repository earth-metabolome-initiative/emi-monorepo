#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CappingProcedureTemplateForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub container_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub procedure_template_container_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub capped_with_model: Option<
        crate::codegen::structs_codegen::tables::caps_models::CapsModel,
    >,
    pub procedure_template_capped_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub capping_pm_compatibility_rules: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate {
    type ForeignKeys = CappingProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                        self.container_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                        self.foreign_procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_container_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CapsModel(
                        self.capped_with_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_capped_with_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                        self.container_model,
                        self.capped_with_model,
                    )),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.container_model.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.procedure_template_container_model.is_some()
            && foreign_keys.capped_with_model.is_some()
            && foreign_keys.procedure_template_capped_with_model.is_some()
            && foreign_keys.capping_pm_compatibility_rules.is_some()
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
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(procedure_templates.clone());
                    updated = true;
                }
                if self.foreign_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.foreign_procedure_template = Some(
                        procedure_templates.clone(),
                    );
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
                if self.foreign_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.foreign_procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(
                    asset_compatibility_rules,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.container_model == asset_compatibility_rules.left_asset_model
                    && self.capped_with_model
                        == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.capping_pm_compatibility_rules = Some(
                        asset_compatibility_rules,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(
                    asset_compatibility_rules,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.container_model == asset_compatibility_rules.left_asset_model
                    && self.capped_with_model
                        == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.capping_pm_compatibility_rules = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CapsModel(caps_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.capped_with_model == caps_models.id {
                    foreign_keys.capped_with_model = Some(caps_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CapsModel(caps_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.capped_with_model == caps_models.id {
                    foreign_keys.capped_with_model = None;
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
                if self.procedure_template_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_container_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_capped_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_capped_with_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_container_model = None;
                    updated = true;
                }
                if self.procedure_template_capped_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_capped_with_model = None;
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
                if self.container_model == volumetric_container_models.id {
                    foreign_keys.container_model = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.container_model == volumetric_container_models.id {
                    foreign_keys.container_model = None;
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
impl web_common_traits::prelude::ForeignKeys for CappingProcedureTemplateForeignKeys {}
