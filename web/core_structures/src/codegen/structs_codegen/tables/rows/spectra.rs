impl From<crate::codegen::structs_codegen::tables::spectra::Spectrum> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::spectra::Spectrum) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>) -> Self {
        super::Rows::Spectrum(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Spectrum(v) => Some(v),
            _ => None,
        }
    }
}
