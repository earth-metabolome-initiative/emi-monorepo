impl From<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
    ) -> Self {
        super::Row::HarvestingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::HarvestingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
