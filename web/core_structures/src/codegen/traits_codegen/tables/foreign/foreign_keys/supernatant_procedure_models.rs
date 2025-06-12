#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SupernatantProcedureModelForeignKeys {
    pub id: Option<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    pub stratified_source: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub supernatant_destination: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
    pub transferred_with: Option<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel {
    type ForeignKeys = SupernatantProcedureModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                        self.id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.stratified_source,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.supernatant_destination,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                        self.transferred_with,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some() && foreign_keys.stratified_source.is_some()
            && foreign_keys.supernatant_destination.is_some()
            && foreign_keys.transferred_with.is_some()
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
                if self.stratified_source == procedure_model_trackables.id {
                    foreign_keys.stratified_source = Some(
                        procedure_model_trackables.clone(),
                    );
                    updated = true;
                }
                if self.supernatant_destination == procedure_model_trackables.id {
                    foreign_keys.supernatant_destination = Some(
                        procedure_model_trackables.clone(),
                    );
                    updated = true;
                }
                if self.transferred_with == procedure_model_trackables.id {
                    foreign_keys.transferred_with = Some(
                        procedure_model_trackables.clone(),
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModelTrackable(
                    procedure_model_trackables,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.stratified_source == procedure_model_trackables.id {
                    foreign_keys.stratified_source = None;
                    updated = true;
                }
                if self.supernatant_destination == procedure_model_trackables.id {
                    foreign_keys.supernatant_destination = None;
                    updated = true;
                }
                if self.transferred_with == procedure_model_trackables.id {
                    foreign_keys.transferred_with = None;
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
impl web_common_traits::prelude::ForeignKeys for SupernatantProcedureModelForeignKeys {}
