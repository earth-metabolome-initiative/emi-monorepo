impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::taxa::Taxon
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTaxon;
}
