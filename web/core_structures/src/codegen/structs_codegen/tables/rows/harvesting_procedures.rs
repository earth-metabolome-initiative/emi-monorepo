impl From<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
        >,
    ) -> Self {
        super::Rows::HarvestingProcedure(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::HarvestingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
