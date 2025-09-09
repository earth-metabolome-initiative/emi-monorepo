#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StorageProcedureTemplateForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub stored_into_model: Option<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    >,
    pub procedure_template_stored_into_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub stored_asset_model: Option<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    >,
    pub procedure_template_stored_asset_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub storage_procedure_templates_stored_into_model_stored_asset_fkey: Option<
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate {
    type ForeignKeys = StorageProcedureTemplateForeignKeys;
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                        self.stored_into_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_stored_into_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                        self.stored_asset_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_stored_asset_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerCompatibilityRule((
                        self.stored_into_model,
                        self.stored_asset_model,
                    )),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && foreign_keys.stored_into_model.is_some()
            && foreign_keys.procedure_template_stored_into_model.is_some()
            && foreign_keys.stored_asset_model.is_some()
            && foreign_keys.procedure_template_stored_asset_model.is_some()
            && foreign_keys
                .storage_procedure_templates_stored_into_model_stored_asset_fkey
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
                crate::codegen::tables::row::Row::ContainerCompatibilityRule(
                    container_compatibility_rules,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.stored_into_model
                    == container_compatibility_rules.container_model
                    && self.stored_asset_model
                        == container_compatibility_rules.contained_asset_model
                {
                    foreign_keys
                        .storage_procedure_templates_stored_into_model_stored_asset_fkey = Some(
                        container_compatibility_rules,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerCompatibilityRule(
                    container_compatibility_rules,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stored_into_model
                    == container_compatibility_rules.container_model
                    && self.stored_asset_model
                        == container_compatibility_rules.contained_asset_model
                {
                    foreign_keys
                        .storage_procedure_templates_stored_into_model_stored_asset_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.stored_into_model == container_models.id {
                    foreign_keys.stored_into_model = Some(container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stored_into_model == container_models.id {
                    foreign_keys.stored_into_model = None;
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
                if self.stored_asset_model == physical_asset_models.id {
                    foreign_keys.stored_asset_model = Some(physical_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(
                    physical_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stored_asset_model == physical_asset_models.id {
                    foreign_keys.stored_asset_model = None;
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
                if self.procedure_template_stored_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stored_into_model = Some(
                        procedure_template_asset_models.clone(),
                    );
                    updated = true;
                }
                if self.procedure_template_stored_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stored_asset_model = Some(
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
                if self.procedure_template_stored_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stored_into_model = None;
                    updated = true;
                }
                if self.procedure_template_stored_asset_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stored_asset_model = None;
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
impl web_common_traits::prelude::ForeignKeys for StorageProcedureTemplateForeignKeys {}
