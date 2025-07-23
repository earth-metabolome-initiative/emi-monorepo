#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliquotingProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
    >,
    pub aliquoted_with: Option<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
    >,
    pub pipette_tip: Option<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    >,
    pub aliquoted_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub aliquoting_procedures_procedure_id_aliquoted_with_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    >,
    pub aliquoting_procedures_procedure_id_pipette_tip_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    >,
    pub aliquoting_procedures_procedure_id_aliquoted_container_id_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure
{
    type ForeignKeys = AliquotingProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                self.procedure_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedureModel(
                self.procedure_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                self.aliquoted_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteTipModel(
                self.pipette_tip,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.aliquoted_container_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable((
                self.procedure_id,
                self.aliquoted_with,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable((
                self.procedure_id,
                self.pipette_tip,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable((
                self.procedure_id,
                self.aliquoted_container_id,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_model.is_some()
            && foreign_keys.aliquoted_with.is_some()
            && foreign_keys.pipette_tip.is_some()
            && foreign_keys.aliquoted_container.is_some()
            && foreign_keys.aliquoting_procedures_procedure_id_aliquoted_with_fkey.is_some()
            && foreign_keys.aliquoting_procedures_procedure_id_pipette_tip_fkey.is_some()
            && foreign_keys.aliquoting_procedures_procedure_id_aliquoted_container_id_fkey.is_some()
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
                crate::codegen::tables::row::Row::AliquotingProcedureModel(
                    aliquoting_procedure_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_model_id == aliquoting_procedure_models.procedure_model_id {
                    foreign_keys.procedure_model = Some(aliquoting_procedure_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AliquotingProcedureModel(
                    aliquoting_procedure_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_model_id == aliquoting_procedure_models.procedure_model_id {
                    foreign_keys.procedure_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.aliquoted_with == pipette_models.id {
                    foreign_keys.aliquoted_with = Some(pipette_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_with == pipette_models.id {
                    foreign_keys.aliquoted_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteTipModel(pipette_tip_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.pipette_tip == pipette_tip_models.id {
                    foreign_keys.pipette_tip = Some(pipette_tip_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteTipModel(pipette_tip_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.pipette_tip == pipette_tip_models.id {
                    foreign_keys.pipette_tip = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTrackable(procedure_trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.aliquoted_with == procedure_trackables.trackable_id
                {
                    foreign_keys.aliquoting_procedures_procedure_id_aliquoted_with_fkey =
                        Some(procedure_trackables);
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.pipette_tip == procedure_trackables.trackable_id
                {
                    foreign_keys.aliquoting_procedures_procedure_id_pipette_tip_fkey =
                        Some(procedure_trackables);
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.aliquoted_container_id == procedure_trackables.trackable_id
                {
                    foreign_keys.aliquoting_procedures_procedure_id_aliquoted_container_id_fkey =
                        Some(procedure_trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTrackable(procedure_trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.aliquoted_with == procedure_trackables.trackable_id
                {
                    foreign_keys.aliquoting_procedures_procedure_id_aliquoted_with_fkey = None;
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.pipette_tip == procedure_trackables.trackable_id
                {
                    foreign_keys.aliquoting_procedures_procedure_id_pipette_tip_fkey = None;
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.aliquoted_container_id == procedure_trackables.trackable_id
                {
                    foreign_keys.aliquoting_procedures_procedure_id_aliquoted_container_id_fkey =
                        None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_id == procedures.id {
                    foreign_keys.procedure = Some(procedures);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_id == procedures.id {
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
                if self.aliquoted_container_id == volumetric_container_models.id {
                    foreign_keys.aliquoted_container = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.aliquoted_container_id == volumetric_container_models.id {
                    foreign_keys.aliquoted_container = None;
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
