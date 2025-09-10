#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CappingProcedureForeignKeys {
    pub capped_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub capping_procedures_capped_container_model_capped_with_mode_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
    pub capped_container_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub capped_with_model: Option<
        crate::codegen::structs_codegen::tables::cap_models::CapModel,
    >,
    pub procedure_capped_container: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure_capped_with: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template_capped_container_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template_capped_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure
{
    type ForeignKeys = CappingProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.capped_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.capped_container_model,
                self.capped_with_model,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.capped_container_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CapModel(
                self.capped_with_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_capped_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_capped_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_capped_container_model,
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
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CappingProcedureTemplate(
                self.procedure_template,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.capped_container.is_some()
            && foreign_keys
                .capping_procedures_capped_container_model_capped_with_mode_fkey
                .is_some()
            && foreign_keys.capped_container_model.is_some()
            && foreign_keys.capped_with_model.is_some()
            && foreign_keys.procedure_capped_container.is_some()
            && foreign_keys.procedure_capped_with.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template_capped_container_model.is_some()
            && foreign_keys.procedure_template_capped_with_model.is_some()
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
                if self.capped_container_model == asset_compatibility_rules.left_asset_model
                    && self.capped_with_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.capping_procedures_capped_container_model_capped_with_mode_fkey =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.capped_container_model == asset_compatibility_rules.left_asset_model
                    && self.capped_with_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.capping_procedures_capped_container_model_capped_with_mode_fkey =
                        None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CapModel(cap_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.capped_with_model == cap_models.id {
                    foreign_keys.capped_with_model = Some(cap_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CapModel(cap_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.capped_with_model == cap_models.id {
                    foreign_keys.capped_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CappingProcedureTemplate(
                    capping_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == capping_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(capping_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CappingProcedureTemplate(
                    capping_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == capping_procedure_templates.procedure_template {
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
                if self.procedure_capped_container == procedure_assets.id {
                    foreign_keys.procedure_capped_container = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_capped_with == procedure_assets.id {
                    foreign_keys.procedure_capped_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_capped_container == procedure_assets.id {
                    foreign_keys.procedure_capped_container = None;
                    updated = true;
                }
                if self.procedure_capped_with == procedure_assets.id {
                    foreign_keys.procedure_capped_with = None;
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
                if self.procedure_template_capped_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_capped_container_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_capped_with_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_capped_with_model =
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
                if self.procedure_template_capped_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_capped_container_model = None;
                    updated = true;
                }
                if self.procedure_template_capped_with_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_capped_with_model = None;
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
                if self.capped_container_model == volumetric_container_models.id {
                    foreign_keys.capped_container_model = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.capped_container_model == volumetric_container_models.id {
                    foreign_keys.capped_container_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.capped_container == volumetric_containers.id {
                    foreign_keys.capped_container = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.capped_container == volumetric_containers.id {
                    foreign_keys.capped_container = None;
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
impl web_common_traits::prelude::ForeignKeys for CappingProcedureForeignKeys {}
