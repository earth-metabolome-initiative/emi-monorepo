#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BallMillProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub foreign_procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub bead_model: Option<
        crate::codegen::structs_codegen::tables::beads_models::BeadsModel,
    >,
    pub milled_with_model: Option<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    >,
    pub milled_with: Option<
        crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine,
    >,
    pub milled_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    >,
    pub ball_mill_procedures_procedure_bead_model_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
    pub ball_mill_procedures_procedure_milled_with_model_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedureTemplate(
                self.procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.foreign_procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                self.foreign_procedure,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BeadsModel(
                self.bead_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachineModel(
                self.milled_with_model,
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                self.milled_container,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset((
                self.procedure,
                self.bead_model,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset((
                self.procedure,
                self.milled_with_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.foreign_procedure.is_some()
            && foreign_keys.bead_model.is_some()
            && foreign_keys.milled_with_model.is_some()
            && (foreign_keys.milled_with.is_some() || self.milled_with.is_some())
            && foreign_keys.milled_container.is_some()
            && foreign_keys.ball_mill_procedures_procedure_bead_model_fkey.is_some()
            && foreign_keys.ball_mill_procedures_procedure_milled_with_model_fkey.is_some()
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
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.foreign_procedure_template == procedure_templates.procedure_template {
                    foreign_keys.foreign_procedure_template = Some(procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.foreign_procedure_template == procedure_templates.procedure_template {
                    foreign_keys.foreign_procedure_template = None;
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
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = Some(procedures);
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
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.milled_with_model == ball_mill_machine_models.id {
                    foreign_keys.milled_with_model = Some(ball_mill_machine_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.milled_with_model == ball_mill_machine_models.id {
                    foreign_keys.milled_with_model = None;
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
                crate::codegen::tables::row::Row::BeadsModel(beads_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.bead_model == beads_models.id {
                    foreign_keys.bead_model = Some(beads_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BeadsModel(beads_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.bead_model == beads_models.id {
                    foreign_keys.bead_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure == procedure_assets.procedure
                    && self.bead_model == procedure_assets.asset_model
                {
                    foreign_keys.ball_mill_procedures_procedure_bead_model_fkey =
                        Some(procedure_assets);
                    updated = true;
                }
                if self.procedure == procedure_assets.procedure
                    && self.milled_with_model == procedure_assets.asset_model
                {
                    foreign_keys.ball_mill_procedures_procedure_milled_with_model_fkey =
                        Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedure_assets.procedure
                    && self.bead_model == procedure_assets.asset_model
                {
                    foreign_keys.ball_mill_procedures_procedure_bead_model_fkey = None;
                    updated = true;
                }
                if self.procedure == procedure_assets.procedure
                    && self.milled_with_model == procedure_assets.asset_model
                {
                    foreign_keys.ball_mill_procedures_procedure_milled_with_model_fkey = None;
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
