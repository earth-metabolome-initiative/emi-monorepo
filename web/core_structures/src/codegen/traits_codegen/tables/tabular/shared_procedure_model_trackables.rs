impl web_common_traits::prelude::Tabular
for crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::SharedProcedureModelTrackable
    }
}
impl web_common_traits::prelude::StaticTabular
for crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::SharedProcedureModelTrackable
    }
}
impl web_common_traits::prelude::Row
for crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::SharedProcedureModelTrackable((
            self.parent_id,
            self.child_id,
        ))
    }
}
