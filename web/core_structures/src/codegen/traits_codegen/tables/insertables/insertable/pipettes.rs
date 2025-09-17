impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::pipettes::Pipette
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePipette;
}
