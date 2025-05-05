impl From<crate::codegen::structs_codegen::tables::units::Unit> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::units::Unit) -> Self {
        super::Row::Unit(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::units::Unit {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Unit(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
