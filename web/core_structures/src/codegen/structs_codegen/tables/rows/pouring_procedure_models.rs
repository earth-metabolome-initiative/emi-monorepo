impl From<crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel,
        >,
    ) -> Self {
        super::Rows::PouringProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PouringProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
