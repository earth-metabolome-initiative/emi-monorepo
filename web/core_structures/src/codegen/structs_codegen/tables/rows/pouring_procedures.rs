impl From<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>,
    ) -> Self {
        super::Rows::PouringProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PouringProcedure(v) => Some(v),
            _ => None,
        }
    }
}
