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
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FreezeDryingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
