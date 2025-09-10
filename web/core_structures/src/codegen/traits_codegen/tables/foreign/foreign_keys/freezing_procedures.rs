#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FreezingProcedureForeignKeys {
    pub frozen_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    >,
    pub frozen_container_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub procedure_template_frozen_container_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_frozen_container: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub frozen_with: Option<crate::codegen::structs_codegen::tables::freezers::Freezer>,
    pub frozen_with_model: Option<
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel,
    >,
    pub procedure_template_frozen_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_frozen_with: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub freezing_procedures_frozen_with_model_frozen_container_mod_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure
{
    type ForeignKeys = FreezingProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.frozen_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezingProcedureTemplate(
                self.procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.frozen_container_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_frozen_container_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_frozen_container,
            ),
        ));
        if let Some(frozen_with) = self.frozen_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Freezer(frozen_with),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezerModel(
                self.frozen_with_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_frozen_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_frozen_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.frozen_with_model,
                self.frozen_container_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.frozen_container.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.frozen_container_model.is_some()
            && foreign_keys.procedure_template_frozen_container_model.is_some()
            && foreign_keys.procedure_frozen_container.is_some()
            && (foreign_keys.frozen_with.is_some() || self.frozen_with.is_some())
            && foreign_keys.frozen_with_model.is_some()
            && foreign_keys.procedure_template_frozen_with_model.is_some()
            && foreign_keys.procedure_frozen_with.is_some()
            && foreign_keys
                .freezing_procedures_frozen_with_model_frozen_container_mod_fkey
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
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.frozen_with_model == asset_compatibility_rules.left_asset_model
                    && self.frozen_container_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.freezing_procedures_frozen_with_model_frozen_container_mod_fkey =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.frozen_with_model == asset_compatibility_rules.left_asset_model
                    && self.frozen_container_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.freezing_procedures_frozen_with_model_frozen_container_mod_fkey =
                        None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezerModel(freezer_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.frozen_with_model == freezer_models.id {
                    foreign_keys.frozen_with_model = Some(freezer_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezerModel(freezer_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.frozen_with_model == freezer_models.id {
                    foreign_keys.frozen_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Freezer(freezers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.frozen_with.is_some_and(|frozen_with| frozen_with == freezers.id) {
                    foreign_keys.frozen_with = Some(freezers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Freezer(freezers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.frozen_with.is_some_and(|frozen_with| frozen_with == freezers.id) {
                    foreign_keys.frozen_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezingProcedureTemplate(
                    freezing_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == freezing_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(freezing_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::FreezingProcedureTemplate(
                    freezing_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == freezing_procedure_templates.procedure_template {
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
                if self.procedure_frozen_container == procedure_assets.id {
                    foreign_keys.procedure_frozen_container = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_frozen_with == procedure_assets.id {
                    foreign_keys.procedure_frozen_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_frozen_container == procedure_assets.id {
                    foreign_keys.procedure_frozen_container = None;
                    updated = true;
                }
                if self.procedure_frozen_with == procedure_assets.id {
                    foreign_keys.procedure_frozen_with = None;
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
                if self.procedure_template_frozen_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_frozen_container_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_frozen_with_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_frozen_with_model =
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
                if self.procedure_template_frozen_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_frozen_container_model = None;
                    updated = true;
                }
                if self.procedure_template_frozen_with_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_frozen_with_model = None;
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
                if self.frozen_container_model == volumetric_container_models.id {
                    foreign_keys.frozen_container_model = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.frozen_container_model == volumetric_container_models.id {
                    foreign_keys.frozen_container_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.frozen_container == volumetric_containers.id {
                    foreign_keys.frozen_container = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.frozen_container == volumetric_containers.id {
                    foreign_keys.frozen_container = None;
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
impl web_common_traits::prelude::ForeignKeys for FreezingProcedureForeignKeys {}
