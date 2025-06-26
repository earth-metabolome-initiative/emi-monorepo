impl web_common_traits::prelude::Tabular
for crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::GeolocationProcedureModel
    }
}
impl web_common_traits::prelude::StaticTabular
for crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::GeolocationProcedureModel
    }
}
impl web_common_traits::prelude::Row
for crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::GeolocationProcedureModel(
            self.procedure_model_id,
        )
    }
}
