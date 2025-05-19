impl From<crate::codegen::structs_codegen::tables::reagents::Reagent> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::reagents::Reagent) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::reagents::Reagent>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::reagents::Reagent>) -> Self {
        super::Rows::Reagent(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::reagents::Reagent> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Reagent(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
