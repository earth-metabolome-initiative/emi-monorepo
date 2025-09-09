impl From<crate::codegen::structs_codegen::tables::containers::Container> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::containers::Container) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::containers::Container>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::containers::Container>) -> Self {
        super::Rows::Container(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::containers::Container> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Container(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
