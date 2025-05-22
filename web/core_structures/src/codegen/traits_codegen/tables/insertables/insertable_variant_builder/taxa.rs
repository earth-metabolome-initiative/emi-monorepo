#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
{
    type Row = crate::codegen::structs_codegen::tables::taxa::Taxon;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTaxon;
}
