impl From<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        >,
    ) -> Self {
        super::Rows::ProcedureTrackable(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureTrackable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
