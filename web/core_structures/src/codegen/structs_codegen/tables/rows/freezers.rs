impl From<crate::codegen::structs_codegen::tables::freezers::Freezer> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::freezers::Freezer) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::freezers::Freezer>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::freezers::Freezer>) -> Self {
        super::Rows::Freezer(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::freezers::Freezer> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Freezer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
