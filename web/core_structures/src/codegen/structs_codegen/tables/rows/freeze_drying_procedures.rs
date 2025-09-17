impl From<crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
        >,
    ) -> Self {
        super::Rows::FreezeDryingProcedure(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezeDryingProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
