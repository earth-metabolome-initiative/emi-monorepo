impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::spectra::Spectrum
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectrum;
}
