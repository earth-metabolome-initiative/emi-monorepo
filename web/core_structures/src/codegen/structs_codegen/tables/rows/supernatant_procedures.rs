impl From<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<Vec<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
        >,
    ) -> Self {
        super::Rows::SupernatantProcedure(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SupernatantProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
