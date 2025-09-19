impl From<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>,
    ) -> Self {
        super::Rows::SpectraCollection(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SpectraCollection(v) => Some(v),
            _ => None,
        }
    }
}
