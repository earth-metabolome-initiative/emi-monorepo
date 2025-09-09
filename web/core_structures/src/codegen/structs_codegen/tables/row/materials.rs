impl From<crate::codegen::structs_codegen::tables::materials::Material> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::materials::Material) -> Self {
        super::Row::Material(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::materials::Material {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Material(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
