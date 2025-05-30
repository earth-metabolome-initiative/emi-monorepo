impl From<crate::codegen::structs_codegen::tables::organizations::Organization> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::organizations::Organization) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organizations::Organization>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::organizations::Organization>,
    ) -> Self {
        super::Rows::Organization(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::organizations::Organization>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Organization(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
