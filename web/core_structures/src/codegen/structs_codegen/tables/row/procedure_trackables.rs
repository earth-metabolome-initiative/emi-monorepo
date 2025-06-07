impl From<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    ) -> Self {
        super::Row::ProcedureTrackable(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureTrackable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
