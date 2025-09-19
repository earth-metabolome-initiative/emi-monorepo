impl From<crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
    ) -> Self {
        super::Row::CentrifugeProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CentrifugeProcedure(v) => Some(v),
            _ => None,
        }
    }
}
