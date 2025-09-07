#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliquotingProcedureForeignKeys {
    pub procedure: Option<crate::Procedure>,
    pub procedure_template: Option<crate::AliquotingProcedureTemplate>,
    pub aliquoted_with: Option<crate::Pipette>,
    pub aliquoted_with_model: Option<crate::PipetteModel>,
    pub procedure_template_aliquoted_with_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_aliquoted_with: Option<crate::ProcedureAsset>,
    pub pipette_tip_model: Option<crate::PipetteTipModel>,
    pub procedure_template_pipette_tip_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_pipette_tip: Option<crate::ProcedureAsset>,
    pub aliquoted_from: Option<crate::VolumetricContainer>,
    pub procedure_template_aliquoted_from_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_aliquoted_from: Option<crate::ProcedureAsset>,
    pub aliquoted_into: Option<crate::VolumetricContainer>,
    pub procedure_template_aliquoted_into_model: Option<crate::ProcedureTemplateAssetModel>,
    pub procedure_aliquoted_into: Option<crate::ProcedureAsset>,
    pub aliquoting_procedures_aliquoted_with_model_pipette_tip_mod_fkey:
        Option<crate::AssetCompatibilityRule>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::AliquotingProcedure {
    type ForeignKeys = AliquotingProcedureForeignKeys;
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
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        if let Some(aliquoted_with) = self.aliquoted_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Pipette(
                    aliquoted_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                self.aliquoted_with_model,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_aliquoted_with_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_aliquoted_with,
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.aliquoted_from,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_aliquoted_from_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_aliquoted_from,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.aliquoted_into,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_aliquoted_into_model,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_aliquoted_into,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.aliquoted_with_model,
                self.pipette_tip_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && (foreign_keys.aliquoted_with.is_some() || self.aliquoted_with.is_some())
            && foreign_keys.aliquoted_with_model.is_some()
            && foreign_keys.procedure_template_aliquoted_with_model.is_some()
            && foreign_keys.procedure_aliquoted_with.is_some()
            && foreign_keys.pipette_tip_model.is_some()
            && foreign_keys.procedure_template_pipette_tip_model.is_some()
            && foreign_keys.procedure_pipette_tip.is_some()
            && foreign_keys.aliquoted_from.is_some()
            && foreign_keys.procedure_template_aliquoted_from_model.is_some()
            && foreign_keys.procedure_aliquoted_from.is_some()
            && foreign_keys.aliquoted_into.is_some()
            && foreign_keys.procedure_template_aliquoted_into_model.is_some()
            && foreign_keys.procedure_aliquoted_into.is_some()
            && foreign_keys
                .aliquoting_procedures_aliquoted_with_model_pipette_tip_mod_fkey
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
                crate::codegen::tables::row::Row::AliquotingProcedureTemplate(
                    aliquoting_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == aliquoting_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(aliquoting_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AliquotingProcedureTemplate(
                    aliquoting_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == aliquoting_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.aliquoted_with_model == asset_compatibility_rules.left_asset_model
                    && self.pipette_tip_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.aliquoting_procedures_aliquoted_with_model_pipette_tip_mod_fkey =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_with_model == asset_compatibility_rules.left_asset_model
                    && self.pipette_tip_model == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.aliquoting_procedures_aliquoted_with_model_pipette_tip_mod_fkey =
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
                if self.aliquoted_with_model == pipette_models.id {
                    foreign_keys.aliquoted_with_model = Some(pipette_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_with_model == pipette_models.id {
                    foreign_keys.aliquoted_with_model = None;
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
                if self.aliquoted_with.is_some_and(|aliquoted_with| aliquoted_with == pipettes.id) {
                    foreign_keys.aliquoted_with = Some(pipettes);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Pipette(pipettes),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_with.is_some_and(|aliquoted_with| aliquoted_with == pipettes.id) {
                    foreign_keys.aliquoted_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_aliquoted_with == procedure_assets.id {
                    foreign_keys.procedure_aliquoted_with = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_pipette_tip == procedure_assets.id {
                    foreign_keys.procedure_pipette_tip = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_aliquoted_from == procedure_assets.id {
                    foreign_keys.procedure_aliquoted_from = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_aliquoted_into == procedure_assets.id {
                    foreign_keys.procedure_aliquoted_into = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_aliquoted_with == procedure_assets.id {
                    foreign_keys.procedure_aliquoted_with = None;
                    updated = true;
                }
                if self.procedure_pipette_tip == procedure_assets.id {
                    foreign_keys.procedure_pipette_tip = None;
                    updated = true;
                }
                if self.procedure_aliquoted_from == procedure_assets.id {
                    foreign_keys.procedure_aliquoted_from = None;
                    updated = true;
                }
                if self.procedure_aliquoted_into == procedure_assets.id {
                    foreign_keys.procedure_aliquoted_into = None;
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
                if self.procedure_template_aliquoted_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_aliquoted_with_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_pipette_tip_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_pipette_tip_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_aliquoted_from_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_aliquoted_from_model =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_aliquoted_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_aliquoted_into_model =
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
                if self.procedure_template_aliquoted_with_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_aliquoted_with_model = None;
                    updated = true;
                }
                if self.procedure_template_pipette_tip_model == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_pipette_tip_model = None;
                    updated = true;
                }
                if self.procedure_template_aliquoted_from_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_aliquoted_from_model = None;
                    updated = true;
                }
                if self.procedure_template_aliquoted_into_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_aliquoted_into_model = None;
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
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.aliquoted_from == volumetric_containers.id {
                    foreign_keys.aliquoted_from = Some(volumetric_containers);
                    updated = true;
                }
                if self.aliquoted_into == volumetric_containers.id {
                    foreign_keys.aliquoted_into = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_from == volumetric_containers.id {
                    foreign_keys.aliquoted_from = None;
                    updated = true;
                }
                if self.aliquoted_into == volumetric_containers.id {
                    foreign_keys.aliquoted_into = None;
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
impl web_common_traits::prelude::ForeignKeys for AliquotingProcedureForeignKeys {}
