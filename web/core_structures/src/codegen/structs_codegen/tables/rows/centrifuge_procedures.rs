impl From<crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
        >,
    ) -> Self {
        super::Rows::CentrifugeProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CentrifugeProcedure(v) => Some(v),
            _ => None,
        }
    }
}
