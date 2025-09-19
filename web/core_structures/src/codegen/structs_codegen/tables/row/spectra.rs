impl From<crate::codegen::structs_codegen::tables::spectra::Spectrum> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::spectra::Spectrum) -> Self {
        super::Row::Spectrum(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::spectra::Spectrum> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Spectrum(v) => Some(v),
            _ => None,
        }
    }
}
