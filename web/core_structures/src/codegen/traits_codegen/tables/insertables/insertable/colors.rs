impl web_common_traits::database::Insertable for crate::Color {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableColorBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableColor;
}
