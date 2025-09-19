impl From<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    ) -> Self {
        super::Row::SpectraCollection(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SpectraCollection(v) => Some(v),
            _ => None,
        }
    }
}
