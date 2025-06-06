#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PouringProcedureModelForeignKeys {
    pub destination: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub id: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub measured_with: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub source: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel
{
    type ForeignKeys = PouringProcedureModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                self.destination,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                self.measured_with,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                self.source,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.destination.is_some()
            && foreign_keys.id.is_some()
            && foreign_keys.measured_with.is_some()
            && foreign_keys.source.is_some()
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
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.destination == procedure_model_trackables.id {
                    foreign_keys.destination = Some(procedure_model_trackables.clone());
                    updated = true;
                }
                if self.measured_with == procedure_model_trackables.id {
                    foreign_keys.measured_with = Some(procedure_model_trackables.clone());
                    updated = true;
                }
                if self.source == procedure_model_trackables.id {
                    foreign_keys.source = Some(procedure_model_trackables.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.destination == procedure_model_trackables.id {
                    foreign_keys.destination = None;
                    updated = true;
                }
                if self.measured_with == procedure_model_trackables.id {
                    foreign_keys.measured_with = None;
                    updated = true;
                }
                if self.source == procedure_model_trackables.id {
                    foreign_keys.source = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == procedure_models.id {
                    foreign_keys.id = Some(procedure_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == procedure_models.id {
                    foreign_keys.id = None;
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
impl web_common_traits::prelude::ForeignKeys for PouringProcedureModelForeignKeys {}
