impl From<
    crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
        >,
    ) -> Self {
        super::Rows::SharedProcedureModelTrackable(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SharedProcedureModelTrackable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
