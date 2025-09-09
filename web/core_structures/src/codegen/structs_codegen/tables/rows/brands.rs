impl From<crate::codegen::structs_codegen::tables::brands::Brand> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::brands::Brand) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::brands::Brand>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::brands::Brand>) -> Self {
        super::Rows::Brand(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::brands::Brand> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Brand(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
