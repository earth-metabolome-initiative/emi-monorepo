impl From<crate::codegen::structs_codegen::tables::units::Unit> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::units::Unit) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::units::Unit>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::units::Unit>) -> Self {
        super::Rows::Unit(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::units::Unit> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Unit(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
