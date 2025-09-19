impl From<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
    ) -> Self {
        super::Row::PackagingProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PackagingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
