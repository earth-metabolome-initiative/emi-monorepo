impl From<crate::codegen::structs_codegen::tables::procedures::Procedure> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::procedures::Procedure) -> Self {
        super::Row::Procedure(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::procedures::Procedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Procedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
