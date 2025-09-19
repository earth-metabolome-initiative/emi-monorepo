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
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::HarvestingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
