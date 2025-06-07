impl From<
    crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
    ) -> Self {
        super::Row::SharedProcedureModelTrackable(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SharedProcedureModelTrackable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
