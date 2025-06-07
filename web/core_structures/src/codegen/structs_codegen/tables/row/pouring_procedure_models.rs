impl From<crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel,
    ) -> Self {
        super::Row::PouringProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PouringProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
