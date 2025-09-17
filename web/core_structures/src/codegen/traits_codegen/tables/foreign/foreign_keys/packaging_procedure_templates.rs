#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackagingProcedureTemplateForeignKeys {
    pub packaged_with_model: Option<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    >,
    pub packaging_procedure_templates_packaged_with_model_sample_m_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub procedure_template_packaged_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template_sample_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub sample_model: Option<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate {
    type ForeignKeys = PackagingProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingModel(
                        self.packaged_with_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                        self.packaged_with_model,
                        self.sample_model,
                    )),
                ),
            );
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_packaged_with_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_sample_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                        self.sample_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.packaged_with_model.is_some()
            && foreign_keys
                .packaging_procedure_templates_packaged_with_model_sample_m_fkey
                .is_some() && foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure_template_packaged_with_model.is_some()
            && foreign_keys.procedure_template_sample_model.is_some()
            && foreign_keys.sample_model.is_some()
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
                crate::codegen::tables::row::Row::AssetCompatibilityRule(
                    asset_compatibility_rules,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.packaged_with_model == asset_compatibility_rules.left_asset_model
                    && self.sample_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys
                        .packaging_procedure_templates_packaged_with_model_sample_m_fkey = Some(
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
                if self.packaged_with_model == asset_compatibility_rules.left_asset_model
                    && self.sample_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys
                        .packaging_procedure_templates_packaged_with_model_sample_m_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PackagingModel(packaging_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.packaged_with_model == packaging_models.id {
                    foreign_keys.packaged_with_model = Some(packaging_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PackagingModel(packaging_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.packaged_with_model == packaging_models.id {
                    foreign_keys.packaged_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(
                    physical_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.sample_model == physical_asset_models.id {
                    foreign_keys.sample_model = Some(physical_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(
                    physical_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.sample_model == physical_asset_models.id {
                    foreign_keys.sample_model = None;
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
                if self.procedure_template_packaged_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_packaged_with_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_sample_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_sample_model = Some(
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
                if self.procedure_template_packaged_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_packaged_with_model = None;
                    updated = true;
                }
                if self.procedure_template_sample_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_sample_model = None;
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
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for PackagingProcedureTemplateForeignKeys {}
