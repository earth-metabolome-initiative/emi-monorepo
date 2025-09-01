impl From<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>,
    ) -> Self {
        super::Rows::CappingProcedure(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CappingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
