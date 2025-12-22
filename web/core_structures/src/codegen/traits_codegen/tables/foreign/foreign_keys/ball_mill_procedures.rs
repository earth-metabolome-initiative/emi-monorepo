#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BallMillProcedureForeignKeys {
    pub bead_model: Option<
        crate::codegen::structs_codegen::tables::bead_models::BeadModel,
    >,
    pub ball_mill_procedures_bead_model_milled_container_model_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
    pub milled_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub milled_container_model: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub milled_with: Option<
        crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine,
    >,
    pub ball_mill_procedures_milled_with_model_bead_model_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
    pub milled_with_model: Option<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    >,
    pub ball_mill_procedures_milled_with_model_milled_container_mo_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
    >,
    pub procedure_bead: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_milled_container: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure_milled_with: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub procedure_template_bead_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    >,
    pub procedure_template_milled_container_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub procedure_template_milled_with_model: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure
{
    type ForeignKeys = BallMillProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BeadModel(
                self.bead_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.bead_model_id,
                self.milled_container_model_id,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.milled_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.milled_container_model_id,
            ),
        ));
        if let Some(milled_with) = self.milled_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachine(
                    milled_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.milled_with_model_id,
                self.bead_model_id,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachineModel(
                self.milled_with_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
                self.milled_with_model_id,
                self.milled_container_model_id,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_bead,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_milled_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                self.procedure_milled_with,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_bead_model_id,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedureTemplate(
                self.procedure_template,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_milled_container_model_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.procedure_template_milled_with_model_id,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.bead_model.is_some()
            && foreign_keys.ball_mill_procedures_bead_model_milled_container_model_fkey.is_some()
            && foreign_keys.milled_container.is_some()
            && foreign_keys.milled_container_model.is_some()
            && (foreign_keys.milled_with.is_some() || self.milled_with.is_some())
            && foreign_keys.ball_mill_procedures_milled_with_model_bead_model_fkey.is_some()
            && foreign_keys.milled_with_model.is_some()
            && foreign_keys
                .ball_mill_procedures_milled_with_model_milled_container_mo_fkey
                .is_some()
            && foreign_keys.procedure_bead.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_milled_container.is_some()
            && foreign_keys.procedure_milled_with.is_some()
            && foreign_keys.procedure_template_bead_model.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.procedure_template_milled_container_model.is_some()
            && foreign_keys.procedure_template_milled_with_model.is_some()
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
                if self.bead_model_id == asset_compatibility_rules.left_asset_model
                    && self.milled_container_model_id == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.ball_mill_procedures_bead_model_milled_container_model_fkey =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
                if self.milled_with_model_id == asset_compatibility_rules.left_asset_model
                    && self.bead_model_id == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.ball_mill_procedures_milled_with_model_bead_model_fkey =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
                if self.milled_with_model_id == asset_compatibility_rules.left_asset_model
                    && self.milled_container_model_id == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.ball_mill_procedures_milled_with_model_milled_container_mo_fkey =
                        Some(asset_compatibility_rules);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetCompatibilityRule(asset_compatibility_rules),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.bead_model_id == asset_compatibility_rules.left_asset_model
                    && self.milled_container_model_id == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.ball_mill_procedures_bead_model_milled_container_model_fkey = None;
                    updated = true;
                }
                if self.milled_with_model_id == asset_compatibility_rules.left_asset_model
                    && self.bead_model_id == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.ball_mill_procedures_milled_with_model_bead_model_fkey = None;
                    updated = true;
                }
                if self.milled_with_model_id == asset_compatibility_rules.left_asset_model
                    && self.milled_container_model_id == asset_compatibility_rules.right_asset_model
                {
                    foreign_keys.ball_mill_procedures_milled_with_model_milled_container_mo_fkey =
                        None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.milled_with_model_id == ball_mill_machine_models.id {
                    foreign_keys.milled_with_model_id = Some(ball_mill_machine_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.milled_with_model_id == ball_mill_machine_models.id {
                    foreign_keys.milled_with_model_id = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachine(ball_mill_machines),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.milled_with.is_some_and(|milled_with| milled_with == ball_mill_machines.id)
                {
                    foreign_keys.milled_with = Some(ball_mill_machines);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachine(ball_mill_machines),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.milled_with.is_some_and(|milled_with| milled_with == ball_mill_machines.id)
                {
                    foreign_keys.milled_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillProcedureTemplate(
                    ball_mill_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == ball_mill_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(ball_mill_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillProcedureTemplate(
                    ball_mill_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == ball_mill_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BeadModel(bead_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.bead_model_id == bead_models.id {
                    foreign_keys.bead_model_id = Some(bead_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BeadModel(bead_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.bead_model_id == bead_models.id {
                    foreign_keys.bead_model_id = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_bead == procedure_assets.id {
                    foreign_keys.procedure_bead = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_milled_container == procedure_assets.id {
                    foreign_keys.procedure_milled_container = Some(procedure_assets);
                    updated = true;
                }
                if self.procedure_milled_with == procedure_assets.id {
                    foreign_keys.procedure_milled_with = Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_bead == procedure_assets.id {
                    foreign_keys.procedure_bead = None;
                    updated = true;
                }
                if self.procedure_milled_container == procedure_assets.id {
                    foreign_keys.procedure_milled_container = None;
                    updated = true;
                }
                if self.procedure_milled_with == procedure_assets.id {
                    foreign_keys.procedure_milled_with = None;
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
                if self.procedure_template_bead_model_id == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_bead_model_id =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_milled_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_milled_container_model_id =
                        Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.procedure_template_milled_with_model_id
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_milled_with_model_id =
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
                if self.procedure_template_bead_model_id == procedure_template_asset_models.id {
                    foreign_keys.procedure_template_bead_model_id = None;
                    updated = true;
                }
                if self.procedure_template_milled_container_model
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_milled_container_model_id = None;
                    updated = true;
                }
                if self.procedure_template_milled_with_model_id
                    == procedure_template_asset_models.id
                {
                    foreign_keys.procedure_template_milled_with_model_id = None;
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
                if self.milled_container_model_id == volumetric_container_models.id {
                    foreign_keys.milled_container_model_id = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.milled_container_model_id == volumetric_container_models.id {
                    foreign_keys.milled_container_model_id = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.milled_container == volumetric_containers.id {
                    foreign_keys.milled_container = Some(volumetric_containers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainer(volumetric_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.milled_container == volumetric_containers.id {
                    foreign_keys.milled_container = None;
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
impl web_common_traits::prelude::ForeignKeys for BallMillProcedureForeignKeys {}
