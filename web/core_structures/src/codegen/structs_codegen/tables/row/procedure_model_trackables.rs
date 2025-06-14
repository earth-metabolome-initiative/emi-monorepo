impl From<
    crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    ) -> Self {
        super::Row::ProcedureModelTrackable(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureModelTrackable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
