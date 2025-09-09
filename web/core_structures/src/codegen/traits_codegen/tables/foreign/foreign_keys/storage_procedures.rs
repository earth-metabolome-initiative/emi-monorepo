#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StorageProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
    >,
    pub stored_asset: Option<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    >,
    pub stored_asset_model: Option<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    >,
    pub procedure_template_stored_asset_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_stored_asset: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub stored_into: Option<
        crate::codegen::structs_codegen::tables::containers::Container,
    >,
    pub stored_into_model: Option<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    >,
    pub procedure_template_stored_into_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_stored_into: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub storage_procedures_stored_into_model_stored_asset_model_fkey: Option<
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure
{
    type ForeignKeys = StorageProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageProcedureTemplate(
                self.procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(
                self.stored_asset,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                self.stored_asset_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_stored_asset_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_stored_asset,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                self.stored_into,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                self.stored_into_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_stored_into_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_stored_into,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerCompatibilityRule(
                (self.stored_into_model, self.stored_asset_model),
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.stored_asset.is_some()
            && foreign_keys.stored_asset_model.is_some()
            && foreign_keys.procedure_template_stored_asset_model.is_some()
            && foreign_keys.procedure_stored_asset.is_some()
            && foreign_keys.stored_into.is_some()
            && foreign_keys.stored_into_model.is_some()
            && foreign_keys.procedure_template_stored_into_model.is_some()
            && foreign_keys.procedure_stored_into.is_some()
            && foreign_keys.storage_procedures_stored_into_model_stored_asset_model_fkey.is_some()
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
                if self.stored_into_model == container_compatibility_rules.container_model
                    && self.stored_asset_model
                        == container_compatibility_rules.contained_asset_model
                {
                    foreign_keys.storage_procedures_stored_into_model_stored_asset_model_fkey =
                        Some(container_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerCompatibilityRule(
                    container_compatibility_rules,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stored_into_model == container_compatibility_rules.container_model
                    && self.stored_asset_model
                        == container_compatibility_rules.contained_asset_model
                {
                    foreign_keys.storage_procedures_stored_into_model_stored_asset_model_fkey =
                        None;
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
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.stored_into == containers.id {
                    foreign_keys.stored_into = Some(containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stored_into == containers.id {
                    foreign_keys.stored_into = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(physical_asset_models),
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
                crate::codegen::tables::row::Row::PhysicalAssetModel(physical_asset_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stored_asset_model == physical_asset_models.id {
                    foreign_keys.stored_asset_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.stored_asset == physical_assets.id {
                    foreign_keys.stored_asset = Some(physical_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stored_asset == physical_assets.id {
                    foreign_keys.stored_asset = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_stored_asset == procedure_assets.id {
                    foreign_keys.procedure_stored_asset = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_stored_into == procedure_assets.id {
                    foreign_keys.procedure_stored_into = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_stored_asset == procedure_assets.id {
                    foreign_keys.procedure_stored_asset = None;
                    updated = true;
                }
                if self.procedure_stored_into == procedure_assets.id {
                    foreign_keys.procedure_stored_into = None;
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
                if self.procedure_template_stored_asset_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stored_asset_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_stored_into_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_stored_into_model =
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
                if self.procedure_template_stored_asset_model == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_stored_asset_model = None;
                    updated = true;
                }
                if self.procedure_template_stored_into_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_stored_into_model = None;
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
                crate::codegen::tables::row::Row::StorageProcedureTemplate(
                    storage_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == storage_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(storage_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::StorageProcedureTemplate(
                    storage_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == storage_procedure_templates.procedure_template {
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
impl web_common_traits::prelude::ForeignKeys for StorageProcedureForeignKeys {}
