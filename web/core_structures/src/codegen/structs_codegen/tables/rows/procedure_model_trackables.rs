impl From<
    crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    ) -> Self {
        super::Rows::ProcedureModelTrackable(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureModelTrackable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
