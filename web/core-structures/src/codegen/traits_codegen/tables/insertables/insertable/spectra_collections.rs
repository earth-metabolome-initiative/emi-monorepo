#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder;
}
