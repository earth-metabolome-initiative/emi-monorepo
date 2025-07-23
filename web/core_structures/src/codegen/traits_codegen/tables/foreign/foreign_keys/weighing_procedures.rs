#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeighingProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
    >,
    pub weighted_with: Option<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
    pub weighted_container: Option<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    pub weighing_procedures_procedure_id_weighted_with_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    >,
    pub weighing_procedures_procedure_id_weighted_container_id_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure
{
    type ForeignKeys = WeighingProcedureForeignKeys;
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedureModel(
                self.procedure_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingInstrumentModel(
                self.weighted_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                self.weighted_container_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable((
                self.procedure_id,
                self.weighted_with,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable((
                self.procedure_id,
                self.weighted_container_id,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_model.is_some()
            && foreign_keys.weighted_with.is_some()
            && foreign_keys.weighted_container.is_some()
            && foreign_keys.weighing_procedures_procedure_id_weighted_with_fkey.is_some()
            && foreign_keys.weighing_procedures_procedure_id_weighted_container_id_fkey.is_some()
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
                crate::codegen::tables::row::Row::ProcedureTrackable(procedure_trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.weighted_with == procedure_trackables.trackable_id
                {
                    foreign_keys.weighing_procedures_procedure_id_weighted_with_fkey =
                        Some(procedure_trackables);
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.weighted_container_id == procedure_trackables.trackable_id
                {
                    foreign_keys.weighing_procedures_procedure_id_weighted_container_id_fkey =
                        Some(procedure_trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTrackable(procedure_trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.weighted_with == procedure_trackables.trackable_id
                {
                    foreign_keys.weighing_procedures_procedure_id_weighted_with_fkey = None;
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.weighted_container_id == procedure_trackables.trackable_id
                {
                    foreign_keys.weighing_procedures_procedure_id_weighted_container_id_fkey = None;
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
                if self.weighted_container_id == volumetric_container_models.id {
                    foreign_keys.weighted_container = Some(volumetric_container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::VolumetricContainerModel(
                    volumetric_container_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.weighted_container_id == volumetric_container_models.id {
                    foreign_keys.weighted_container = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingInstrumentModel(
                    weighing_instrument_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.weighted_with == weighing_instrument_models.id {
                    foreign_keys.weighted_with = Some(weighing_instrument_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingInstrumentModel(
                    weighing_instrument_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.weighted_with == weighing_instrument_models.id {
                    foreign_keys.weighted_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingProcedureModel(weighing_procedure_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_model_id == weighing_procedure_models.procedure_model_id {
                    foreign_keys.procedure_model = Some(weighing_procedure_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingProcedureModel(weighing_procedure_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_model_id == weighing_procedure_models.procedure_model_id {
                    foreign_keys.procedure_model = None;
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
impl web_common_traits::prelude::ForeignKeys for WeighingProcedureForeignKeys {}
