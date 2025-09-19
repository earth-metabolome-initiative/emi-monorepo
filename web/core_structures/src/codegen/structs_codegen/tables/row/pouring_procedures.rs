impl From<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
    ) -> Self {
        super::Row::PouringProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PouringProcedure(v) => Some(v),
            _ => None,
        }
    }
}
