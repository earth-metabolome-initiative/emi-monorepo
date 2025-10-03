impl From<crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure,
    ) -> Self {
        super::Row::TaggingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::TaggingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
