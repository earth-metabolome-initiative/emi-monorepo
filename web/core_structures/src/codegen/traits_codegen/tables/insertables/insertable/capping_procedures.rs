impl web_common_traits::database::Insertable for crate::CappingProcedure {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedure;
}
