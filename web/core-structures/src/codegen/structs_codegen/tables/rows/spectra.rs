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
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Spectrum(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
