impl From<crate::codegen::structs_codegen::tables::materials::Material> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::materials::Material) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::materials::Material>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::materials::Material>) -> Self {
        super::Rows::Material(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::materials::Material> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Material(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
