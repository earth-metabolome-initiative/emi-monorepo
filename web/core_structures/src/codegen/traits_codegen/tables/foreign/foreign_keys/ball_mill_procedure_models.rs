#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BallMillProcedureModelForeignKeys {
    pub procedure_model: Option<
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    >,
    pub milled_with: Option<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel
{
    type ForeignKeys = BallMillProcedureModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageProcedureModel(
                self.procedure_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachineModel(
                self.milled_with,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_model.is_some() && foreign_keys.milled_with.is_some()
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
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.milled_with == ball_mill_machine_models.id {
                    foreign_keys.milled_with = Some(ball_mill_machine_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.milled_with == ball_mill_machine_models.id {
                    foreign_keys.milled_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::StorageProcedureModel(storage_procedure_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_model_id == storage_procedure_models.procedure_model_id {
                    foreign_keys.procedure_model = Some(storage_procedure_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::StorageProcedureModel(storage_procedure_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_model_id == storage_procedure_models.procedure_model_id {
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
impl web_common_traits::prelude::ForeignKeys for BallMillProcedureModelForeignKeys {}
