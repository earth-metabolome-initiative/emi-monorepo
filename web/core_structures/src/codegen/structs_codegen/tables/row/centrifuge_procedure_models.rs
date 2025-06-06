impl From<
    crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
    ) -> Self {
        super::Row::CentrifugeProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CentrifugeProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
