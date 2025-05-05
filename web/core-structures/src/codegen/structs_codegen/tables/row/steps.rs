impl From<crate::codegen::structs_codegen::tables::steps::Step> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::steps::Step) -> Self {
        super::Row::Step(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::steps::Step {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Step(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
