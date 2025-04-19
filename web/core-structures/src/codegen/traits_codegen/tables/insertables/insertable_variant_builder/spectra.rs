#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder
{
    type Row = crate::codegen::structs_codegen::tables::spectra::Spectrum;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableSpectrum;
}
