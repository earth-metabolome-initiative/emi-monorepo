#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder
{
    type Row = crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection;
}
