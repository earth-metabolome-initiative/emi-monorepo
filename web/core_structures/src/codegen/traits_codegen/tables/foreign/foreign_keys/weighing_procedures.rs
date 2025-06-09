#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeighingProcedureForeignKeys {
    pub instrument: Option<crate::codegen::structs_codegen::tables::trackables::Trackable>,
    pub procedure: Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
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
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                self.instrument_id,
            ),
        ));
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
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.instrument.is_some()
            && foreign_keys.procedure.is_some()
            && foreign_keys.procedure_model.is_some()
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
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.instrument_id == trackables.id {
                    foreign_keys.instrument = Some(trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.instrument_id == trackables.id {
                    foreign_keys.instrument = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingProcedureModel(weighing_procedure_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_model_id == weighing_procedure_models.id {
                    foreign_keys.procedure_model = Some(weighing_procedure_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::WeighingProcedureModel(weighing_procedure_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_model_id == weighing_procedure_models.id {
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
