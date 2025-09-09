#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SupernatantProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    >,
    pub stratified_source: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub procedure_template_stratified_source_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_stratified_source: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub supernatant_destination: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub procedure_template_supernatant_destination_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_supernatant_destination: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub transferred_with: Option<
        crate::codegen::structs_codegen::tables::pipettes::Pipette,
    >,
    pub transferred_with_model: Option<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
    >,
    pub procedure_template_transferred_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_transferred_with: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub pipette_tip_model: Option<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    >,
    pub procedure_template_pipette_tip_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_pipette_tip: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub supernatant_procedures_transferred_with_model_pipette_tip_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure
{
    type ForeignKeys = SupernatantProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::SupernatantProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.stratified_source,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_stratified_source_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_stratified_source,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.supernatant_destination,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_supernatant_destination_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_supernatant_destination,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Pipette(
                self.transferred_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                self.transferred_with_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_transferred_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_transferred_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteTipModel(
                self.pipette_tip_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_pipette_tip_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_pipette_tip,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.transferred_with_model,
                self.pipette_tip_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.stratified_source.is_some()
            && foreign_keys.procedure_template_stratified_source_model.is_some()
            && foreign_keys.procedure_stratified_source.is_some()
            && foreign_keys.supernatant_destination.is_some()
            && foreign_keys.procedure_template_supernatant_destination_model.is_some()
            && foreign_keys.procedure_supernatant_destination.is_some()
            && foreign_keys.transferred_with.is_some()
            && foreign_keys.transferred_with_model.is_some()
            && foreign_keys.procedure_template_transferred_with_model.is_some()
            && foreign_keys.procedure_transferred_with.is_some()
            && foreign_keys.pipette_tip_model.is_some()
            && foreign_keys.procedure_template_pipette_tip_model.is_some()
            && foreign_keys.procedure_pipette_tip.is_some()
            && foreign_keys.supernatant_procedures_transferred_with_model_pipette_tip_fkey.is_some()
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
                    foreign_keys.supernatant_procedures_transferred_with_model_pipette_tip_fkey =
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
                    foreign_keys.supernatant_procedures_transferred_with_model_pipette_tip_fkey =
                        None;
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
                crate::codegen::tables::row::Row::Pipette(pipettes),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.transferred_with == pipettes.id {
                    foreign_keys.transferred_with = Some(pipettes);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Pipette(pipettes),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.transferred_with == pipettes.id {
                    foreign_keys.transferred_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_stratified_source == procedure_assets.id {
                    foreign_keys.procedure_stratified_source = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_supernatant_destination == procedure_assets.id {
                    foreign_keys.procedure_supernatant_destination = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_transferred_with == procedure_assets.id {
                    foreign_keys.procedure_transferred_with = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_pipette_tip == procedure_assets.id {
                    foreign_keys.procedure_pipette_tip = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_stratified_source == procedure_assets.id {
                    foreign_keys.procedure_stratified_source = None;
                    updated = true;
                }
                if self.procedure_supernatant_destination == procedure_assets.id {
                    foreign_keys.procedure_supernatant_destination = None;
                    updated = true;
                }
                if self.procedure_transferred_with == procedure_assets.id {
                    foreign_keys.procedure_transferred_with = None;
                    updated = true;
                }
                if self.procedure_pipette_tip == procedure_assets.id {
                    foreign_keys.procedure_pipette_tip = None;
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
                if self.procedure_template_pipette_tip_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_pipette_tip_model =
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
                if self.procedure_template_pipette_tip_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_pipette_tip_model = None;
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
                crate::codegen::tables::row::Row::SupernatantProcedureTemplate(
                    supernatant_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == supernatant_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(supernatant_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SupernatantProcedureTemplate(
                    supernatant_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == supernatant_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.stratified_source == volumetric_containers.id {
                    foreign_keys.stratified_source = Some(volumetric_containers);
                    updated = true;
                }
                if self.supernatant_destination == volumetric_containers.id {
                    foreign_keys.supernatant_destination = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stratified_source == volumetric_containers.id {
                    foreign_keys.stratified_source = None;
                    updated = true;
                }
                if self.supernatant_destination == volumetric_containers.id {
                    foreign_keys.supernatant_destination = None;
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
impl web_common_traits::prelude::ForeignKeys for SupernatantProcedureForeignKeys {}
