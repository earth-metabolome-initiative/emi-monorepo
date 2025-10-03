impl From<crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure>,
    ) -> Self {
        super::Rows::TaggingProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::TaggingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
