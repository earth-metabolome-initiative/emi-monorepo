#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::spectra::Spectrum
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectrum;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder;
}
