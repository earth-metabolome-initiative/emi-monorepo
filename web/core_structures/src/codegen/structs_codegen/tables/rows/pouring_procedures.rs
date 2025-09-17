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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PouringProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
