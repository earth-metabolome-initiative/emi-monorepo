impl web_common_traits::database::Insertable for crate::Freezer {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezer;
}
