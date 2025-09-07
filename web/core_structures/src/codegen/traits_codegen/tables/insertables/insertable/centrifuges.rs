impl web_common_traits::database::Insertable for crate::Centrifuge {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifuge;
}
