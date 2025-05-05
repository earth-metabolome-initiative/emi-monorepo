impl From<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::document_formats::DocumentFormat,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>,
    ) -> Self {
        super::Rows::DocumentFormat(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DocumentFormat(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
