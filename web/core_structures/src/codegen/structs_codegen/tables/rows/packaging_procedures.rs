impl From<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
        >,
    ) -> Self {
        super::Rows::PackagingProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PackagingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
