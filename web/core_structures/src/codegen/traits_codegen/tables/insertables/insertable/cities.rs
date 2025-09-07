impl web_common_traits::database::Insertable for crate::City {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCity;
}
