impl From<crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<Vec<crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure,
        >,
    ) -> Self {
        super::Rows::RegisteringProcedure(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::registering_procedures::RegisteringProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::RegisteringProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
