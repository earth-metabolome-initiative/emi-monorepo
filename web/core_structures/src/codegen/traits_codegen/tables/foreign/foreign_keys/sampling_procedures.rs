#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SamplingProcedureForeignKeys {
    pub procedure: Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
    pub sampling_procedures_procedure_id_processable_id_fkey:
        Option<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable>,
    pub sampling_procedures_procedure_id_trackable_id_fkey:
        Option<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable>,
    pub processable: Option<crate::codegen::structs_codegen::tables::processables::Processable>,
    pub trackable: Option<crate::codegen::structs_codegen::tables::trackables::Trackable>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure
{
    type ForeignKeys = SamplingProcedureForeignKeys;
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable((
                self.procedure_id,
                self.processable_id,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable((
                self.procedure_id,
                self.trackable_id,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Processable(
                self.processable_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                self.trackable_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.sampling_procedures_procedure_id_processable_id_fkey.is_some()
            && foreign_keys.sampling_procedures_procedure_id_trackable_id_fkey.is_some()
            && foreign_keys.processable.is_some()
            && foreign_keys.trackable.is_some()
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
                    && self.processable_id == procedure_trackables.trackable_id
                {
                    foreign_keys.sampling_procedures_procedure_id_processable_id_fkey =
                        Some(procedure_trackables);
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.trackable_id == procedure_trackables.trackable_id
                {
                    foreign_keys.sampling_procedures_procedure_id_trackable_id_fkey =
                        Some(procedure_trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTrackable(procedure_trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.processable_id == procedure_trackables.trackable_id
                {
                    foreign_keys.sampling_procedures_procedure_id_processable_id_fkey = None;
                    updated = true;
                }
                if self.procedure_id == procedure_trackables.procedure_id
                    && self.trackable_id == procedure_trackables.trackable_id
                {
                    foreign_keys.sampling_procedures_procedure_id_trackable_id_fkey = None;
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
                crate::codegen::tables::row::Row::Processable(processables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.processable_id == processables.id {
                    foreign_keys.processable = Some(processables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Processable(processables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.processable_id == processables.id {
                    foreign_keys.processable = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.trackable_id == trackables.id {
                    foreign_keys.trackable = Some(trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.trackable_id == trackables.id {
                    foreign_keys.trackable = None;
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
impl web_common_traits::prelude::ForeignKeys for SamplingProcedureForeignKeys {}
